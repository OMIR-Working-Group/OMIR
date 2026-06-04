# RFC-0001: Chunk

- **Status:** Draft
- **Author(s):** OMIR Working Group
- **Created:** 2026-06-03
- **Target release:** R2
- **Proposed OMM level on acceptance:** 0 (Draft)
- **Affects:** Reference pattern? **Yes** · Bundle.entry oneOf? **Yes** · common.schema.json? **No** (new resource schema only)

> **RFC number is provisional.** `0001` is a placeholder; the TSC assigns the canonical number at
> intake ([GOVERNANCE §3.2](../../GOVERNANCE.md)). No TSC is seated yet (see
> [global-standard P-1](../src/global-standard.md)), so this RFC is parked as a Draft until the
> founding TSC and the R2 line exist. It is the formal write-up of
> [`efficiency.md` EP-6a](../src/efficiency.md); read that for the watts/inference rationale and the
> divergence (D8) it advances.

## 1. Summary

A **`Chunk`** is a *consolidation product*: a compressed, abstracted unit of memory that
`composedOf` references the `MemoryRecord` / `Episode` / `Entity` resources it consolidates. It
makes the **episodic→semantic derivation first-class** — the transformation the literature
(Complementary Learning Systems; reflection; note-evolution) treats as memory's central operation,
which R1 records only weakly via `parentId` / `entityRefs` (divergence **D8**).

A **reusable** Chunk (`reusable: true`) **is a Template** — a schema/pattern that *new* memories
instantiate (via `MemoryRecord.schemaType`) rather than a one-off abstraction. `Template` is **not**
a separate resource; it is the reusable mode of `Chunk`.

## 2. Motivation

What portable memory data cannot be represented today, by the core resources or an `extension[]`?

- **The derivation itself.** A consolidated fact ("the team uses Rust") summarizing fifty episodic
  records has no first-class way to say *which* records it abstracts and that it is an abstraction,
  not a leaf observation. `parentId` is a single hierarchical link, not a many-to-many `composedOf`.
- **Reusable schemas/templates** (chunking & expertise — Miller; Gobet & Simon templates) — a named
  slot-pattern new memories attach to for cheap schema-consistent integration (Tse et al. 2007) —
  have no representation at all.

Why a **new core type** rather than an `extension[]`? Because `composedOf` is a set of typed
cross-resource references that must satisfy **reference integrity** (CR-5) and participate in the
graph the same way `Relationship` and `entityRefs` do — an opaque `extension[].valueJson` blob is
explicitly **excluded** from the closed-world reference walk
([global-standard Phase 2](../src/global-standard.md#the-phases)), so a chunk-as-extension could
carry dangling references a validator could never catch. A consolidation product is a first-class
graph citizen, not vendor payload.

**The consolidation *event* is deliberately NOT modeled here.** A `ConsolidationEvent`/`Reflection`
*process* record was floated in [HANDOFF §5](../../HANDOFF.md) but is rejected: it re-imports the
consolidation *algorithm* that [`semantics.md`](../src/semantics.md) ("what OMIR deliberately does
not specify") puts out of scope, and duplicates W3C PROV. The derivation is carried as a **Theme-E
provenance hop** on the Chunk — `wasGeneratedBy { activity: "consolidation" }` + `wasDerivedFrom`
over `composedOf` — never a second resource. (The Veld reference implementation concurs
structurally: its event-sourced journal `IntentPayload` is `Remember/Forget/Update/Anchor` with no
consolidation-event variant; consolidation lands its output as `Remember`/`Update` of records.)

## 3. Resource model

- `resourceType` const: `"Chunk"`.

| Field | Type | Constraint | Required | Description |
|---|---|---|:---:|---|
| `resourceType` | const | `"Chunk"` | ✓ | Discriminator. |
| `id` | `Id` | `^[A-Za-z0-9._:-]{1,128}$` | ✓ | Resource-local id, unique within `Chunk`. |
| `meta` | `Meta` | — | | Standard metadata envelope. |
| `content` | string | — | ✓ | The compressed/abstracted unit — a named template, schema, or expert chunk. |
| `composedOf` | array&lt;`Reference`&gt; | each resolves in-Bundle (CR-5) | | The `MemoryRecord`/`Episode`/`Entity` resources this chunk consolidates (the D8 derivation, first-class). |
| `reusable` | boolean | default `false` | | `true` ⇒ this Chunk is a **Template**: a reusable schema/pattern new memories instantiate, not a one-off abstraction. |
| `schemaType` | string | open vocabulary (→ CodeableConcept under Theme A) | | The schema/pattern class this Chunk represents. A `MemoryRecord.schemaType` equal to this value attaches that memory to this Template. |
| `confidence` | `Confidence` | — | | Calibrated belief in the abstraction. |
| `provenance` | `Provenance` | — | | Origin/trust; the consolidation derivation rides here (Theme-E chain when present). |
| `createdAt` | `Instant` | RFC 3339 | ✓ | Encoding time. |
| `extension` | array&lt;`Extension`&gt; | — | | Typed escape hatch. |

Reuses `common.schema.json#/$defs`: `Id`, `Meta`, `Reference`, `Confidence`, `Provenance`,
`Instant`, `Extension`. Referenced **by** nothing in R1 by default; references **out** to
`MemoryRecord`/`Episode`/`Entity` via `composedOf` (typed `Reference`s, closed-world).

## 4. JSON Schema

Draft schema to be added under `schemas/Chunk.schema.json`, satisfying CONTRIBUTING §3 (draft
2020-12, canonical `$id`, `additionalProperties: false`, typed references, documented fields,
minimal `required`). The candidate body is given inline in
[`efficiency.md` EP-6a](../src/efficiency.md); it is not committed to `schemas/` until the R2 line
exists (committing it now would add a non-R1 resource to the validated R1 schema set).

## 5. Examples

A worked example bundle exercising `Chunk` (with both a one-off and a `reusable` Template, plus the
`wasGeneratedBy` consolidation provenance hop) ships under `examples/` **only once the R2 schema set
exists** — an R2 bundle is non-conformant to R1 (`Chunk` is an unknown `resourceType`/`entry` type
under R1's closed schemas), so it cannot enter the R1 conformance corpus (CONTRIBUTING §6). Until
then the worked example lives illustratively in [`efficiency.md`](../src/efficiency.md).

## 6. Heritage / prior art

- **Cognitive science:** chunking & expertise (Miller 1956; Gobet & Simon templates); schema-fast
  integration (Tse et al. 2007); Complementary Learning Systems episodic→semantic consolidation
  (McClelland et al. 1995); reflection as recursive abstraction.
- **AI agent memory:** Generative Agents' *reflection* (higher-level insights from recent memories);
  A-MEM's *note evolution*; HippoRAG's consolidated associative index. See the peer comparison in
  [`memory_theory.md` §7](../../memory_theory.md).
- **Veld:** the compression/consolidation pipeline emits consolidated facts as memories; a `Chunk`
  is the at-rest serialization of that product.

## 7. Backward compatibility & migration

Additive. Adding the `Chunk` resource **invalidates no existing R1 bundle** — older consumers
encounter an unknown `entry` item and, per the consumer rules, ignore it without rejecting. The
breaking surface is the **shared `Reference` pattern** + `Bundle.entry` `oneOf` widening (§5.2),
which is why this lands on the **R2** line, not as an R1.x increment. The migration note enumerates:
`Reference.pattern` now admits `Chunk/<id>`; `Bundle.entry.items.oneOf` gains the `Chunk` schema;
the validator's `RESOURCE_TYPES` / registry-walker / "core resources" prose gain `Chunk`; the
JSON-LD `@context` registers `Chunk`.

## 8. Maturity (OMM) justification

**OMM-0 (Draft).** A brand-new core resource with one reference implementation's consolidation
output as its only field evidence. It cannot claim OMM-1+ until at least one independent
implementation exercises it (per [global-standard P0/P0a](../src/global-standard.md)); the shape is
expected to change (e.g. whether `Template` warrants promotion to its own type — see §9).

## 9. Alternatives considered

1. **`MemoryRecord.composedOf` only (no new resource).** Add `composedOf` to `MemoryRecord` and let a
   "chunk" be a `MemoryRecord` with that field set. *Cheaper* (§5.1-additive, R1.x). **Rejected as
   the sole answer** because a reusable Template carries slot/schema identity that is a different
   ontological kind from a remembered fact; collapsing both onto `MemoryRecord` loses the
   product-vs-leaf distinction. *Retained as a complementary option:* a producer that only needs the
   one-off-derivation case MAY use `MemoryRecord.composedOf` without a `Chunk`.
2. **A separate `Template` resource.** Two resources (`Chunk` for products, `Template` for patterns).
   **Rejected:** they share `composedOf`, `content`, `schemaType`, and lifecycle; a `reusable` flag
   on one resource is the smaller surface (Templates are reusable Chunks).
3. **A `ConsolidationEvent` resource.** **Rejected** — models the process OMIR excludes and
   duplicates W3C PROV; carried as a provenance `wasGeneratedBy` hop instead (§2).
4. **An `extension[]` on `MemoryRecord`.** **Rejected** — `composedOf` needs closed-world reference
   integrity the extension lane explicitly does not provide (§2).

## 10. Open questions

- Should `Template` (reusable Chunk) eventually graduate to its own resource if slot-filler structure
  is added (variables, typed slots)? Deferred until a second implementer demands it.
- Does `composedOf` permit `Chunk → Chunk` (hierarchical chunks of chunks)? Proposed **yes** (a
  `Chunk` is a valid `Reference` target once it is a resource), but it introduces cycles a validator
  SHOULD detect — defer the cycle rule to the R2 validator work.
- Is the consolidation provenance hop (`wasGeneratedBy { activity: "consolidation" }`) **required**
  on a `Chunk`, or optional? Leaning optional (a producer may not track it), with a SHOULD.
