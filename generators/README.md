# OMIR Spec Page Generator

**Status: DESIGN DOC — not yet implemented.** This document specifies a tool
that does not exist in the repository today. It describes the intended design of
`omir-gen`, the generator that will render the human-readable resource pages of
the OMIR R1 specification from the canonical JSON Schemas. Nothing here should be
read as a description of working code; the directory currently contains this
design note only.

---

## Purpose

`schemas/` is the **single source of truth** for OMIR R1. The `*.schema.json`
files (JSON Schema draft 2020-12) define every resource's fields, types,
cardinality, enumerations, and descriptions. They are normative.

The human-readable specification pages under `spec/src/resources/*.md` —
the field tables, cardinality columns, and worked examples a reader sees on
omir.io — are a **generated projection** of those schemas. They are
**never hand-edited**. A field that is not in the schema cannot appear in the
spec page; a field in the schema always does. This keeps the prose spec and the
machine-validatable schema from drifting apart, which is the failure mode that
quietly kills interoperability standards.

This mirrors the convention already used by the reference implementation
(Veld — "Agentic Memory"), whose
`docs/src/architecture/module-index.md` is auto-generated from source by a
standalone Rust generator (`docs/generators/`, e.g. `gen-module-index.rs`) and
carries a "do not hand-edit" banner. OMIR adopts the same discipline for the
same reason: one authoritative artifact, everything else derived.

> **Rule:** edit a schema, regenerate. Never patch a generated `.md` by hand.
> A CI check (see *CI & determinism*) fails the build if the committed pages do
> not match a fresh generation.

---

## Language & license

- **Language:** Rust (2021 edition), as a small standalone Cargo workspace under
  `generators/`, declared with its own `[workspace]` so it does not get pulled
  into any parent workspace. This matches the Veld docs-generator pattern.
- **License:** **Apache-2.0**. The generator is *reference code*, not spec text.
  Per the OMIR licensing split, the **spec and schemas are CC-BY-4.0** and the
  **reference code (validator, generators) is Apache-2.0**. These licenses are
  deliberately decoupled from the reference implementation's BUSL-1.1 core: a
  standard whose tooling is restrictively licensed is dead on arrival.

The crate is named `omir-gen` and is `publish = false`.

---

## Inputs and outputs

### Inputs

| Input | Path | Role |
|---|---|---|
| Resource schemas | `schemas/MemoryRecord.schema.json`, `schemas/Entity.schema.json`, `schemas/Relationship.schema.json`, `schemas/Episode.schema.json`, `schemas/Bundle.schema.json` | One spec page generated per resource schema. |
| Common definitions | `schemas/common.schema.json` | Resolved for `$ref` targets (`Id`, `Reference`, `Instant`, `UnitInterval`, `Meta`, `Extension`, `Confidence`, `Decay`, `Provenance`). Rendered as shared sub-types, not as a top-level resource page. |
| Examples | `examples/*.omir` | Worked examples embedded into pages (see *Examples*). |

Resource schemas are recognized by `"resourceType": { "const": "<Name>" }` at
`properties.resourceType.const`. `common.schema.json` has no `resourceType` and
is treated as the shared-definitions source rather than a resource.

### Outputs

| Output | Path |
|---|---|
| One field-table page per resource | `spec/src/resources/memoryrecord.md`, `entity.md`, `relationship.md`, `episode.md`, `bundle.md` |
| Shared sub-types reference | `spec/src/resources/common.md` (the `$defs` from `common.schema.json`) |

Each generated file begins with a banner:

```text
<!-- GENERATED FILE — DO NOT EDIT. -->
<!-- Source: schemas/<Resource>.schema.json (+ examples/). -->
<!-- Regenerate: cargo run -p omir-gen -- --check=false -->
```

### Page anatomy

Each resource page contains, in order:

1. **Title** from the schema's `title` (e.g. "OMIR MemoryRecord (R1)").
2. **Description** from the schema's top-level `description`. The trailing OMM
   level stated there (e.g. "OMM level 4.") is surfaced as a **Maturity** badge:
   MemoryRecord OMM-4; Entity, Relationship, Episode OMM-3. The generator does
   not invent a level — it reads what the schema declares and does not
   overstate stability.
3. **Field table** (see below).
4. **Examples** pulled from `examples/` (see below).

---

## Field table

For every entry in the schema's `properties`, the generator emits one row:

| Column | Source |
|---|---|
| **Field** | the property key |
| **Type** | resolved type — `string`, `number`, `integer`, `boolean`, `object`, `array<T>`, an `enum` (rendered as the inline value list), a `const` value, or a `$ref` rendered as a link to its definition (e.g. `Reference`, `UnitInterval`, `Confidence` → `common.md#reference`) |
| **Card.** | cardinality string — see derivation rules below |
| **Description** | the property's `description`, verbatim from the schema |

`$ref` types are resolved one hop: a `$ref` into `common.schema.json#/$defs/X`
renders as a link to the `X` sub-type on `common.md`, and `array` items that are
`$ref`s render as `array<X>` with the same link. The generator does **not**
inline-expand referenced sub-schemas into the parent table; it links to them,
keeping each page flat and the common types defined once.

`enum`/`const` values are rendered literally so the page is exhaustive (e.g.
`tier`: `working | session | longterm | archive`; `resourceType`: const
`MemoryRecord`). Defaults declared via `"default"` are noted in the Description
column as `(default: <value>)` when present.

---

## Cardinality derivation rules

Cardinality is computed purely from the schema — never authored by hand:

1. **Required scalar/object** — the field name appears in the schema's
   top-level `required[]` **and** is not an `array` → **`1..1`**.
2. **Array** — the field's resolved `type` is `array` (regardless of whether it
   is required) → **`0..*`**. (R1 schemas do not set `minItems`; if a future
   schema adds `minItems: N`, the generator renders `N..*`.)
3. **Optional scalar/object** — everything else (not in `required[]`,
   not an array) → **`0..1`**.

Worked against the R1 schemas:

- `MemoryRecord.required = ["resourceType","id","content","createdAt"]`
  → those four are `1..1`; `entityRefs`/`extension` are arrays → `0..*`;
  `kind`, `tier`, `importance`, `confidence`, `decay`, `validUntil`, etc.
  → `0..1`.
- `Relationship.required = ["resourceType","id","from","to","relationType"]`
  → those five `1..1`; `extension` → `0..*`; `strength`, `context`,
  `validAt`, `invalidatedAt`, `sourceEpisode` → `0..1`.
- `Bundle.required = ["resourceType","omirVersion","entry"]`
  → `resourceType`/`omirVersion` `1..1`; `entry` is an array → `0..*`
  (semantically the carrier of resources); `@context`, `id`, `generatedAt`,
  `source` → `0..1`.

The rules are intentionally mechanical: required-and-array still derives
`0..*` (an array is the unit of repetition; `required` only asserts the key is
present, e.g. `entry`), so the table never claims a lower bound the schema does
not enforce.

---

## Examples

Examples are pulled from `examples/`, not embedded in prose:

- The generator loads every `examples/*.omir` Bundle and walks `entry[]`.
- For a resource page, it selects matching entries by `resourceType` and embeds
  the first matching entry (pretty-printed JSON) as that page's **Example**
  block. The Bundle page embeds a whole `*.omir` document (R1 ships
  `examples/minimal-bundle.omir`).
- Each embedded block is annotated with its provenance
  (`from examples/minimal-bundle.omir`) so a reader can find the source file.
- Examples are **validated before embedding**: an example that does not conform
  to its schema is a hard error, so the spec can never ship an invalid example.
  (This reuses the same JSON Schema check the OMIR validator performs; until the
  validator crate exists, the generator performs structural validation inline.)

If a resource type has no example in `examples/`, the page renders without an
Example block and the generator emits a warning — it does not fabricate one.

---

## Planned CLI invocation

Following the Veld generator convention (run via Cargo from the repo root):

```bash
# Regenerate every spec/src/resources/*.md from schemas/ + examples/.
cargo run -p omir-gen

# Explicit, equivalent invocation (manifest-path form, no workspace assumptions):
cargo run --bin omir-gen --manifest-path generators/Cargo.toml

# CI / pre-commit mode: regenerate in memory and diff against the committed
# pages. Exit non-zero if they differ. Writes nothing.
cargo run -p omir-gen -- --check

# Point at non-default locations (defaults shown).
cargo run -p omir-gen -- \
  --schemas  schemas/ \
  --examples examples/ \
  --out      spec/src/resources/
```

| Flag | Default | Meaning |
|---|---|---|
| `--schemas <dir>` | `schemas/` | Directory of `*.schema.json` inputs. |
| `--examples <dir>` | `examples/` | Directory of `*.omir` example Bundles. |
| `--out <dir>` | `spec/src/resources/` | Output directory for generated pages. |
| `--check` | off | Verify-only: regenerate in memory, diff against committed files, exit non-zero on mismatch. Writes nothing. |

All paths are resolved relative to the repo root
(`c:/Repositories/Portll/omir-standard`).

---

## CI & determinism

- Output is **deterministic**: properties render in schema declaration order
  (preserving JSON object order), with no timestamps or environment-dependent
  content in the generated files, so `--check` is stable across machines.
- A pre-commit hook runs `omir-gen --check` (cheap) so a schema edit without a
  regenerate is caught before it lands.
- CI runs the same `--check`. If a generated page differs from a fresh
  generation, the build fails with instructions to run `cargo run -p omir-gen`
  and commit the result.

---

## Non-goals (R1)

- The generator does **not** emit the schemas — schemas are hand-authored
  ground truth; the generator only *consumes* them.
- It does **not** generate the JSON-LD `@context`
  (`https://omir.io/spec/R1/context.jsonld`) — that is a separate artifact.
- It does **not** validate Bundles for production use — that is the OMIR
  validator's job (separate Apache-2.0 reference tool). The generator's
  validation is limited to guarding the examples it embeds.
- It does **not** render Profiles or constraint overlays in R1; Profile-aware
  page generation is deferred to a later release.
