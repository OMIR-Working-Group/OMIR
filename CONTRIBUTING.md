# Contributing to OMIR

OMIR — **Open Memory Interoperability Resources** (pronounced *"OH-meer"*) — is an open,
vendor-neutral, at-rest data format for portable AI agent / cognitive memory. It is a
document standard, not a product and not a wire protocol. MCP and A2A transport memory;
**OMIR *is* the memory at rest.**

This repository holds the **specification, JSON Schemas, examples, and reference code**.
It is stewarded by the **OMIR Working Group** under an open RFC + ballot process. Veld
convenes the group but does not own the standard. Everyone is welcome to contribute under
the governance and licensing terms below.

> **Current release:** OMIR R1.

---

## Table of contents

1. [Licensing of contributions](#1-licensing-of-contributions)
2. [Repository layout & the generated-pages rule](#2-repository-layout--the-generated-pages-rule)
3. [Schema style rules](#3-schema-style-rules)
4. [Proposing a new Resource (RFC)](#4-proposing-a-new-resource-rfc)
5. [Proposing or registering an Extension](#5-proposing-or-registering-an-extension)
6. [Adding an example to the conformance corpus](#6-adding-an-example-to-the-conformance-corpus)
7. [Branch / PR / DCO flow](#7-branch--pr--dco-flow)
8. [Maturity model (OMM)](#8-maturity-model-omm)
9. [Governance](#9-governance)

---

## 1. Licensing of contributions

OMIR keeps its license **deliberately decoupled** from any single implementation. A
standard under a restrictive license is dead on arrival; the spec must be free.

| Artifact | License |
|---|---|
| Specification prose & JSON Schemas (`spec/`, `schemas/`) | **CC-BY-4.0** |
| Reference code (validator, generators, tooling) | **Apache-2.0** |

By submitting a contribution you agree that:

- Your changes to **spec text or schemas** are licensed under **CC-BY-4.0**.
- Your changes to **reference code** are licensed under **Apache-2.0**.
- You have the right to submit the work (see the [DCO](#7-branch--pr--dco-flow)).

Do **not** copy text or code into this repository under any other license without prior
TSC approval.

---

## 2. Repository layout & the generated-pages rule

```
omir-standard/
├── schemas/            # JSON Schema (draft 2020-12) — the GROUND TRUTH for every field
│   ├── common.schema.json
│   ├── Bundle.schema.json
│   ├── MemoryRecord.schema.json
│   ├── Entity.schema.json
│   ├── Relationship.schema.json
│   └── Episode.schema.json
├── spec/
│   └── src/
│       └── resources/  # GENERATED human-readable resource pages — DO NOT HAND-EDIT
├── examples/           # Conformance corpus — *.omir / *.omirb sample bundles
└── CONTRIBUTING.md
```

### The schemas are the single source of truth

Every field, type, constraint, and enum is defined **only** in `schemas/`. Documentation,
tooling, and downstream implementations derive from those files. If prose and schema ever
disagree, **the schema wins** and the prose is a bug.

### `spec/src/resources/` is GENERATED — never hand-edit it

The human-readable resource pages under **`spec/src/resources/`** are **generated from
`schemas/`** by the reference generator. They are build output, not source.

- **Do not** hand-edit any file under `spec/src/resources/`. Your edit will be silently
  overwritten on the next regeneration, and the PR will be rejected.
- To change what a resource page says, **edit the corresponding schema** (its `title`,
  `description`, field `description`s, enums, and constraints) and regenerate.
- The `description` fields in the schemas are authored content — they are the prose that
  flows into the generated pages. Write them carefully.

---

## 3. Schema style rules

All schemas in `schemas/` MUST follow these rules. PRs that violate them will not pass review.

1. **Draft 2020-12.** Every schema declares
   `"$schema": "https://json-schema.org/draft/2020-12/schema"`.

2. **Canonical `$id`.** Each schema has an `$id` of the form
   `https://omir.io/spec/R1/schemas/<Name>.schema.json`.

3. **`additionalProperties: false`** on every object schema, including nested objects and
   `$defs`. The core is a closed 80/20 model; anything not in the core rides in the
   typed `extension[]` array, never as a loose extra property. (Maps that are deliberately
   open — e.g. `attributes`, `metadata` — use
   `"additionalProperties": { "type": "string" }`, which is explicit, not loose.)

4. **References are typed, FHIR-style.** A cross-resource link is **never** a bare string.
   It is a `Reference` object: `{ "ref": "ResourceType/id" }`, where `ResourceType` is one
   of `MemoryRecord | Entity | Relationship | Episode` and `id` matches the `Id` pattern.
   Reuse `common.schema.json#/$defs/Reference`; do not redefine reference shapes locally.

   ```json
   "from": { "$ref": "common.schema.json#/$defs/Reference" }
   ```

   ```json
   { "ref": "Entity/john" }
   ```

5. **Reuse `common.schema.json`.** Shared sub-types live in `common.schema.json#/$defs`
   and MUST be reused by `$ref`, never copy-pasted:
   `Id`, `Reference`, `Instant`, `UnitInterval`, `Meta`, `Extension`, `Confidence`,
   `Decay`, `Provenance`.

6. **Identifiers.** Resource-local `id`s use `common.schema.json#/$defs/Id`
   (`^[A-Za-z0-9._:-]{1,128}$`), unique within their `resourceType` inside a Bundle.

7. **Timestamps.** All instants use `common.schema.json#/$defs/Instant`
   (RFC 3339 / ISO 8601, `format: date-time`). UTC is RECOMMENDED.

8. **Normalized scores** (importance, salience, strength, credibility, calibrated
   confidence, …) use `common.schema.json#/$defs/UnitInterval` — a `number` in `[0, 1]`.

9. **Every resource carries `resourceType` and `id`.** `resourceType` is a `const` equal
   to the resource name. List the truly mandatory fields — and only those — in `required`.
   Keep the required set minimal; portability beats strictness.

10. **Enums are lowercase**, `snake_case` where multi-word
    (e.g. `code_edit`, `file_access`). Open vocabularies (e.g. `relationType`) are typed
    `string` with the common values documented in `description`, plus an explicit note that
    implementations MAY mint new lowercase `snake_case` values.

11. **`meta`** (`common.schema.json#/$defs/Meta`) MAY appear on any resource and carries
    `omirVersion` (`"R1"`), `profile[]`, `source`, `createdAt`, `lastUpdated`, and
    `maturity` (the OMM level — see §8).

12. **Document, don't assume.** Every property has a `description`. That description is the
    text that ships in the generated resource page, so write it for a reader who has never
    seen the schema.

Run the reference validator over `schemas/` and `examples/` before opening a PR (see §6).

---

## 4. Proposing a new Resource (RFC)

R1 core resources are **MemoryRecord, Entity, Relationship, Episode**, plus the **Bundle**
container. Adding a *new* core resource type is a significant change: it widens the
`Reference` pattern, the `Bundle.entry` `oneOf`, and the conformance surface. It therefore
goes through the RFC + ballot process and starts at a **low maturity level** (typically
OMM-0 or OMM-1 — see §8). Do not overclaim stability for a brand-new resource.

### Process

1. **Open a discussion / issue first.** Float the idea with the Working Group before
   writing a schema. Confirm the need cannot be met by an `extension[]` on an existing
   resource (the 80/20 rule favors extensions over new core types).
2. **Write an RFC** using the skeleton below and place it under `spec/rfcs/` as
   `RFC-<nnnn>-<short-slug>.md` (the TSC assigns the number).
3. **Include a draft schema** under `schemas/` following every rule in §3, and at least
   one example bundle exercising it under `examples/` (see §6).
4. **Open a PR** linking the RFC, schema, and examples (see §7).
5. **Ballot.** The TSC and Working Group review; acceptance follows the open ballot process.
   Accepted resources land at their graded OMM level and are added to the `Reference`
   pattern and `Bundle.entry` `oneOf`.

### RFC template skeleton

Copy this into `spec/rfcs/RFC-<nnnn>-<slug>.md`:

```markdown
# RFC-<nnnn>: <Resource name>

- **Status:** Draft | In Ballot | Accepted | Rejected | Withdrawn
- **Author(s):** <name> <<email>>
- **Created:** <YYYY-MM-DD>
- **Target release:** R<n>
- **Proposed OMM level on acceptance:** <0–5>  <!-- new resources start low; see §8 -->
- **Affects:** Reference pattern? Bundle.entry oneOf? common.schema.json?

## 1. Summary
One paragraph: what this resource is and the at-rest memory concept it captures.

## 2. Motivation
What real, portable memory data cannot be represented today by the core resources or by
an `extension[]` on an existing resource? Why does this warrant a new *core* type rather
than an extension?

## 3. Resource model
- `resourceType` const value.
- Field-by-field definition: name, type, constraint, whether required, one-line description.
- Which `common.schema.json#/$defs` sub-types it reuses (Id, Reference, Instant,
  UnitInterval, Meta, Extension, Confidence, Decay, Provenance).
- How it is referenced by, and references, the existing resources
  (`ResourceType/id` typed references only).

## 4. JSON Schema
Link the draft schema added under `schemas/<Name>.schema.json`. It MUST satisfy §3:
draft 2020-12, canonical `$id`, `additionalProperties: false`, typed references,
documented enums, minimal `required`.

## 5. Examples
Link the example bundle(s) added under `examples/` that exercise the new resource and
validate cleanly against the full schema set.

## 6. Heritage / prior art
Where this concept already exists in practice (e.g. the Veld reference implementation, FHIR,
other memory systems). Honest mapping, not marketing.

## 7. Backward compatibility & migration
Effect on existing bundles. New core types are additive; older consumers MAY ignore unknown
`entry` items. State explicitly that no R1 bundle becomes invalid.

## 8. Maturity (OMM) justification
Why the proposed OMM level (see §8) is honest given field experience and stability.

## 9. Alternatives considered
Including the "make it an extension instead" option and why it was rejected.

## 10. Open questions
```

---

## 5. Proposing or registering an Extension

Extensions are the **typed escape hatch** that lets implementations carry proprietary or
experimental data **without breaking core conformance**. Consumers MAY ignore unknown
extensions. Prefer an extension over a new core resource whenever possible — that is the
80/20 design at work.

An extension is an entry in a resource's `extension[]` array, conforming to
`common.schema.json#/$defs/Extension`:

```json
{
  "url": "https://veld.dev/omir/ext/scoring-signals",
  "valueJson": { "graphStrength": 0.84, "arousal": 0.6, "feedbackMomentum": 0.71 }
}
```

### Rules

1. **`url` is required and MUST be a canonical, dereferenceable URL** that *you* control and
   that *defines* the extension. Namespace it under a domain you own
   (e.g. `https://veld.dev/omir/ext/...`). Never invent a URL on someone else's domain.
2. **Pick exactly one value shape** per extension: `valueString`, `valueNumber`,
   `valueBoolean`, or `valueJson` (for structured payloads). Do not add new `value*`
   properties — `Extension` is `additionalProperties: false`.
3. **Extensions never change core meaning.** A conformant consumer that drops every
   extension MUST still get a valid, usable bundle. If your data changes how the core is
   interpreted, it is not an extension — it is an RFC (see §4).
4. **Document it** at the `url`: the value shape, semantics, and units.

### Registering an extension (optional but encouraged)

Public, reusable extensions MAY be listed in the community **extension registry** so others
can discover and reuse them instead of minting duplicates.

1. Add an entry under `spec/extensions/` as
   `<vendor>-<short-name>.md` describing the `url`, value shape, semantics, and owner.
2. Add at least one example bundle under `examples/` that uses the extension (§6).
3. Open a PR (§7). Registration is documentation only — it does **not** make the extension
   part of the core, and it does not raise any resource's OMM level.

> Heritage extensions already carried by the format include Veld's 20-signal scoring,
> external isotropy/closure/density dimensions, and name embeddings (embeddings are **not**
> core in R1 and ride on `extension[]`).

---

## 6. Adding an example to the conformance corpus

The `examples/` directory is the **conformance corpus**: sample bundles that every
validator and generator is tested against. Good examples are how we prove the spec is
implementable and catch regressions.

### What makes a good example

- A complete **Bundle** document (`resourceType: "Bundle"`, `omirVersion: "R1"`, `entry[]`).
- File extension **`.omir`** for canonical JSON / JSON-LD, **`.omirb`** for the compact
  binary (CBOR/bincode) profile. Never use `.mf` or `.mif`.
- Realistic but **synthetic** data — no real user PII, secrets, tokens, or credentials.
- Internally consistent **typed references**: every `{ "ref": "ResourceType/id" }` resolves
  to an `entry` actually present in the bundle (or is intentionally dangling for a
  negative-test fixture, clearly named as such).
- Honest `meta.maturity` values per resource type (§8).

### Steps

1. Create the bundle under `examples/`, e.g. `examples/<short-descriptive-name>.omir`.
   Mirror the structure and field conventions of `examples/minimal-bundle.omir`.
2. **Validate it against the full schema set** with the reference validator before opening
   the PR. It MUST validate cleanly against `schemas/Bundle.schema.json` (which composes
   every resource schema). A bundle that does not validate cannot be merged into the corpus.
3. If the example demonstrates a **new** resource (§4) or extension (§5), reference the
   relevant RFC / registry entry in the PR.
4. Open a PR (§7).

> **Negative fixtures** (intentionally invalid bundles, used to test that validators reject
> bad input) are welcome. Place them in a clearly named `examples/invalid/` path and state
> in the PR exactly which rule each one is designed to violate.

---

## 7. Branch / PR / DCO flow

### Branching

- Fork the repository (or branch within it if you are a maintainer).
- Use a descriptive branch name: `rfc/<nnnn>-<slug>`, `schema/<resource>-<change>`,
  `ext/<vendor>-<name>`, `example/<name>`, or `docs/<topic>`.
- Keep PRs focused: one RFC, one schema change, or one coherent set of examples per PR.

### Pull requests

- Target the default branch.
- In the PR description, state **what** changed and **why**, and link any related RFC,
  extension registry entry, or issue.
- **Touch `schemas/` for behavior, not `spec/src/resources/`.** If your PR contains
  hand-edits to generated pages under `spec/src/resources/`, it will be sent back. Edit the
  schema and regenerate instead (§2).
- Confirm the reference validator passes over `schemas/` and `examples/`.
- Schema and example changes typically require **two maintainer approvals**; new core
  resources additionally require the RFC ballot (§4).

### Versioning

OMIR uses **semantic, FHIR-style release versioning** (R1, R2, …). Changes that would break
existing R1 bundles are **not** accepted into R1; they are queued for the next release per
the published deprecation policy. Additive, backward-compatible changes (new optional
fields, new extensions, new examples) are the norm within a release.

### Developer Certificate of Origin (DCO)

All commits MUST be **signed off** under the
[Developer Certificate of Origin 1.1](https://developercertificate.org/). The sign-off
certifies that you wrote the contribution or otherwise have the right to submit it under the
licenses in §1.

Add a sign-off line to every commit:

```
Signed-off-by: Jane Doe <jane@example.com>
```

The easiest way is the `-s` flag:

```bash
git commit -s -m "schema(Episode): clarify eventTime vs createdAt"
```

The name and email in the sign-off MUST match the commit author. PRs with unsigned commits
will be blocked until every commit is signed off.

---

## 8. Maturity model (OMM)

OMIR grades stability with the **OMIR Maturity Model (OMM)** — integer levels **0–5 per
resource TYPE**, surfaced in `meta.maturity` and in the resource's schema/spec page.

| Level | Meaning |
|---|---|
| 0 | Draft — newly proposed; shape may change freely. |
| 1–2 | Trial use — implemented somewhere, but limited field experience. |
| 3 | Established — multiple implementations, real usage, stabilizing. |
| 4 | Mature — broad field experience; changes are conservative. |
| 5 | Normative — stable; breaking changes only across releases per the deprecation policy. |

**Grade honestly.** Current R1 levels:

- **MemoryRecord** — OMM-4.
- **Entity, Relationship, Episode** — OMM-3.
- **Brand-new resources** — start at OMM-0/1.

A PR must not raise a resource's OMM level without field evidence; maturity reflects reality,
not ambition.

---

## 9. Governance

- OMIR is stewarded by a **vendor-neutral OMIR Working Group** in a neutral GitHub org.
  **Veld convenes the group but does not own the standard.**
- Changes flow through an **open RFC + ballot** process overseen by a **Technical Steering
  Committee (TSC)**.
- Releases follow **semantic, FHIR-style versioning** (R1, R2, …) with a **published
  deprecation policy**.
- Be excellent to one another — all participation is governed by our
  [Code of Conduct](CODE_OF_CONDUCT.md).

Welcome aboard, and thank you for helping make agent memory portable.
