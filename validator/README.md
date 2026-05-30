# omir-validate — Reference Conformance Validator (DESIGN DOC)

> **Status: DESIGN — NOT YET IMPLEMENTED.**
> This document specifies the intended behavior of the `omir-validate` reference
> CLI. The tool described here does **not exist yet**; nothing in this README
> should be read as a description of shipped software. It is the design contract
> the implementation will be built against, published ahead of the code so the
> OMIR Working Group can review the conformance rules before they are frozen.

`omir-validate` is the **reference implementation** of the OMIR R1 conformance
suite: a single, dependency-light Rust CLI that decides whether a `.omir` (or
`.omirb`) document is a conformant OMIR R1 Bundle. It is reference code, not part
of any product, and is licensed **Apache-2.0** — decoupled from the OMIR
specification text and JSON Schemas, which are **CC-BY-4.0**.

Being the reference validator, its output is normative in the practical sense:
the "Powered by OMIR" badge (see [Badge gating](#powered-by-omir-badge-gating))
is awarded only to artifacts and producers that pass `omir-validate` at the
**Core** conformance level.

---

## 1. Scope

OMIR is an **at-rest** data format. `omir-validate` validates documents — files
on disk — not wire traffic and not running systems. It answers one question:

> Is this file a valid OMIR R1 Bundle, and to what conformance level?

It does **not** evaluate retrieval quality, memory usefulness, or runtime
behavior. It is a structural and referential conformance checker, in the spirit
of `fhir validate`, the ONNX checker, and `jsonschema` CLIs.

Ground truth for every structural rule is the published JSON Schema set
(draft 2020-12) at `schemas/`:

| Schema file | Resource |
|---|---|
| `common.schema.json` | shared `$defs` (`Reference`, `Meta`, `Confidence`, `Decay`, `Provenance`, …) |
| `Bundle.schema.json` | the `.omir` document container |
| `MemoryRecord.schema.json` | `MemoryRecord` (OMM-4) |
| `Entity.schema.json` | `Entity` (OMM-3) |
| `Relationship.schema.json` | `Relationship` (OMM-3) |
| `Episode.schema.json` | `Episode` (OMM-3) |

The validator embeds a pinned copy of the R1 schemas at build time so it can run
fully offline (no network fetch of `$id` / `$ref` URLs). `--schema-dir` overrides
the embedded set for testing against a draft schema revision.

---

## 2. What it validates

Validation runs as four ordered **check groups**. Group 1 is mandatory and gates
the rest per entry; groups 2–4 run over whatever parsed successfully so a single
run surfaces as many findings as possible.

### 2.1 Structural validation (per entry, against `schemas/*.schema.json`)

1. **Parse.** Decode the document. `.omir` → JSON (UTF-8, JSON-LD compatible);
   `.omirb` → CBOR. A decode failure is a single fatal finding (`E000`).
2. **Bundle envelope.** Validate the top-level object against
   `Bundle.schema.json`: `resourceType == "Bundle"`, `omirVersion == "R1"`,
   `entry` present and an array, `additionalProperties: false`. An optional
   `@context` is permitted (and, if a string, SHOULD equal
   `https://omir.io/spec/R1/context.jsonld`).
3. **Per-entry dispatch.** For each `entry[i]`, read its `resourceType` and
   validate the entry against the matching resource schema. This enforces, for
   each resource:
   - required fields (e.g. `MemoryRecord` requires
     `resourceType, id, content, createdAt`; `Relationship` requires
     `resourceType, id, from, to, relationType`),
   - `additionalProperties: false` (unknown top-level keys are violations —
     proprietary data MUST ride in the typed `extension[]`),
   - enum / `const` constraints (`kind`, `experienceType`, `tier`, entity
     `labels`, episode `source`),
   - value-range constraints from `common.schema.json` (`UnitInterval` ∈ [0,1]
     for `importance`, `salience`, `strength`, `credibility`, `confidence.calibrated`;
     `Confidence.alpha`/`beta` `exclusiveMinimum: 0`; `Decay.halfLifeHours > 0`),
   - `Instant` fields are RFC 3339 / ISO 8601 (`createdAt`, `eventTime`,
     `validUntil`, `validAt`, `invalidatedAt`, `lastSeenAt`, …),
   - `Id` and `Reference` lexical patterns
     (`^[A-Za-z0-9._:-]{1,128}$`; refs `^(MemoryRecord|Entity|Relationship|Episode)/…$`).
   An entry whose `resourceType` is not one of the four R1 resource types is a
   violation (`E101`) — `Bundle.entry[]` is a closed `oneOf` in R1.

### 2.2 Reference integrity (cross-entry)

Every typed reference of the form `"ResourceType/id"` MUST resolve to an entry
present **in the same Bundle**. The validator builds an index of
`(resourceType, id)` over all entries, then walks every reference-bearing field
and confirms the target exists:

| Field | Reference must target |
|---|---|
| `MemoryRecord.entityRefs[]` | an `Entity` |
| `MemoryRecord.parentId` | a `MemoryRecord` (note: `parentId` is a bare `Id`, resolved within `MemoryRecord`) |
| `Episode.entityRefs[]` | an `Entity` |
| `Relationship.from` / `Relationship.to` | an `Entity` |
| `Relationship.sourceEpisode` | an `Episode` |

Findings:

- `E200` **Dangling reference** — `ref` does not resolve to any entry. (error)
- `E201` **Type mismatch** — `ref` resolves to an entry of the wrong
  `resourceType` (e.g. `Relationship.from` pointing at a `MemoryRecord`). (error)
- `W202` **Duplicate id within a resourceType** — two entries share
  `(resourceType, id)`; references become ambiguous. (warning at Core, error at
  Strict)

R1 is **closed-world** for references: an unresolved ref is a failure, not a
deferred lookup. There is no cross-Bundle resolution in R1; a Bundle is
self-contained.

### 2.3 Version presence (`meta.omirVersion == "R1"`)

OMIR R1 requires the release marker to be present and correct so that a consumer
can reject documents from a future, incompatible release without guessing.

- **Bundle level:** `Bundle.omirVersion` MUST `== "R1"` (already enforced by the
  envelope schema in 2.1; re-asserted here as finding `E300` for a clear,
  dedicated message rather than a generic schema error).
- **Resource level:** every entry's `meta.omirVersion`, **when `meta` is
  present**, MUST `== "R1"`. A `meta.omirVersion` that is missing-but-`meta`-present,
  or set to anything other than `"R1"`, is finding `E301`.
- **Missing `meta` entirely:** `meta` is optional in R1 schemas, so its absence
  is **not** an error. At Core it raises `I302` (info) recommending a `meta`
  block; at Strict, `meta` with `omirVersion` is **required** on every entry and
  its absence becomes `E302`.

### 2.4 Profile conformance (optional)

A **Profile** constrains the 80/20 core — narrowing cardinalities, pinning
enums, requiring otherwise-optional fields, or mandating specific extensions —
without changing resource shapes. Profile checks run only when requested:

- `--profile <url-or-path>` loads one or more profile definitions.
- A resource claims conformance to a profile by listing its canonical URL in
  `meta.profile[]`. With `--profile`, the validator checks each claiming
  resource against that profile's additional constraints.
- `--require-profile <url>` makes a profile mandatory: every resource of the
  profiled type that does **not** declare the URL in `meta.profile[]` is a
  finding.

Findings: `E400` profile constraint violated; `E401` resource claims a profile
URL that was not supplied to the run (cannot verify); `W402` profile required by
`--require-profile` but not claimed by an eligible resource.

> Profiles are themselves an OMIR resource family slated for a later release; in
> R1 the validator consumes profile definitions but the **profile schema is
> OMM-1**. Profile checking is therefore strictly opt-in and never affects the
> Core badge.

---

## 3. Conformance levels

`omir-validate` reports against a named level (`--level`, default `core`):

| Level | What must hold | Gates the badge? |
|---|---|---|
| `core` | Groups 2.1 (structural), 2.2 (reference integrity), 2.3 (version presence). Warnings allowed. | **Yes — "Powered by OMIR".** |
| `strict` | Core **plus**: `W202` → error, `meta.omirVersion` required on every entry (`E302`), no warnings tolerated. | No (super-set badge; informational) |
| `profile` | Core **plus** the profiles named by `--profile` / `--require-profile`. | No (per-profile) |

"Core conformance" is precisely a clean `--level core` run: zero findings of
severity `error`.

---

## 4. Findings & severities

Every finding carries a stable machine code, a severity, a JSON Pointer locating
the offending node, and a human message.

| Severity | Meaning | Affects pass/fail |
|---|---|---|
| `error` | Conformance violation. | Yes — any `error` ⇒ overall `fail`. |
| `warning` | Conformant but risky / discouraged. | At `core`: no. At `strict`: yes. |
| `info` | Advisory / recommendation. | No. |

Code ranges: `E0xx` parse/IO, `E1xx` structural, `E2xx`/`W2xx` reference
integrity, `E3xx`/`I3xx` version presence, `E4xx`/`W4xx` profile.

---

## 5. Conformance report

The report is the validator's product. Two renderings of the **same** underlying
report object are emitted via `--format {human,json}` (default `human`).
`--format json` writes a machine-readable report to stdout; `-o <file>` writes
to a file.

### 5.1 JSON report (canonical)

The JSON form is normative — it is what badge automation and CI consume. It will
itself be schema-described at `schemas/ConformanceReport.schema.json` once the
tool is implemented (also CC-BY-4.0). Shape:

```json
{
  "report": "OMIRConformanceReport",
  "omirValidateVersion": "0.1.0",
  "schemaRelease": "R1",
  "target": "examples/minimal-bundle.omir",
  "level": "core",
  "result": "pass",
  "summary": {
    "entriesTotal": 5,
    "entriesByType": { "MemoryRecord": 1, "Entity": 2, "Relationship": 1, "Episode": 1 },
    "errors": 0,
    "warnings": 0,
    "info": 0
  },
  "checks": {
    "structural": "pass",
    "referenceIntegrity": "pass",
    "versionPresence": "pass",
    "profile": "skipped"
  },
  "findings": [],
  "coreConformant": true,
  "badgeEligible": true
}
```

Per-entry findings appear in `findings[]`, each anchored to its entry:

```json
{
  "findings": [
    {
      "code": "E200",
      "severity": "error",
      "check": "referenceIntegrity",
      "entry": { "index": 3, "resourceType": "Relationship", "id": "rel-varun-omir" },
      "path": "/entry/3/to/ref",
      "value": "Entity/omr",
      "message": "Dangling reference: 'Entity/omr' does not resolve to any entry in the Bundle. Did you mean 'Entity/omir'?"
    },
    {
      "code": "E301",
      "severity": "error",
      "check": "versionPresence",
      "entry": { "index": 0, "resourceType": "Episode", "id": "ep-launch-chat" },
      "path": "/entry/0/meta/omirVersion",
      "value": "R0",
      "message": "meta.omirVersion must be \"R1\" when meta is present (found \"R0\")."
    }
  ]
}
```

When `result` is `fail`, `coreConformant` and `badgeEligible` are `false`.

### 5.2 Human report

```
omir-validate 0.1.0 — OMIR R1 conformance
target: examples/minimal-bundle.omir   level: core

  structural ............ pass   (5 entries: 1 MemoryRecord, 2 Entity, 1 Relationship, 1 Episode)
  reference integrity ... pass   (6 references resolved)
  version presence ...... pass   (Bundle=R1, 5/5 entries meta.omirVersion=R1)
  profile ............... skipped (no --profile supplied)

RESULT: PASS  —  Core conformant  ✓  (badge-eligible)
```

A failing run lists findings grouped by entry, e.g.:

```
  reference integrity ... FAIL

  Relationship/rel-varun-omir  (entry 3)
    E200  error   /entry/3/to/ref
          Dangling reference: 'Entity/omr' does not resolve to any entry.
          Did you mean 'Entity/omir'?

RESULT: FAIL  —  1 error, 0 warnings  —  NOT Core conformant (badge withheld)
```

### 5.3 Exit codes

| Code | Meaning |
|---|---|
| `0` | `pass` at the requested level. |
| `1` | `fail` — one or more findings count against the level. |
| `2` | Tool/usage error (bad flags, unreadable file, unparseable input — `E000`). |

CI gates on the exit code; dashboards consume the JSON.

---

## 6. CLI usage

```
omir-validate <FILE>... [OPTIONS]
```

Validate the bundled example at Core (the default level), human output:

```
$ omir-validate examples/minimal-bundle.omir
omir-validate 0.1.0 — OMIR R1 conformance
target: examples/minimal-bundle.omir   level: core

  structural ............ pass   (5 entries: 1 MemoryRecord, 2 Entity, 1 Relationship, 1 Episode)
  reference integrity ... pass   (6 references resolved)
  version presence ...... pass   (Bundle=R1, 5/5 entries meta.omirVersion=R1)
  profile ............... skipped (no --profile supplied)

RESULT: PASS  —  Core conformant  ✓  (badge-eligible)
$ echo $?
0
```

Emit the canonical JSON report for CI / badge automation:

```
$ omir-validate examples/minimal-bundle.omir --format json -o report.json
$ jq '.result, .coreConformant, .badgeEligible' report.json
"pass"
true
true
```

Other invocations:

```
# Strict level — promotes warnings to errors, requires meta.omirVersion everywhere
$ omir-validate examples/minimal-bundle.omir --level strict

# Profile conformance — verify claimed profiles and require one
$ omir-validate bundle.omir \
    --profile https://omir.io/profiles/robotics-edge \
    --require-profile https://omir.io/profiles/robotics-edge

# Compact binary profile
$ omir-validate snapshot.omirb

# Validate many files; non-zero exit if any fail
$ omir-validate exports/*.omir --format json -o reports/

# Test against a draft schema revision instead of the embedded R1 set
$ omir-validate bundle.omir --schema-dir ../omir-standard/schemas
```

### 6.1 Options

| Flag | Default | Effect |
|---|---|---|
| `--level {core,strict,profile}` | `core` | Conformance level to grade against. |
| `--format {human,json}` | `human` | Report rendering. |
| `-o, --out <path>` | stdout | Write the report to a file (or a directory when validating many files). |
| `--profile <url\|path>` | — | Load a profile definition (repeatable). |
| `--require-profile <url>` | — | Require the named profile on eligible resources. |
| `--schema-dir <dir>` | embedded R1 | Override the embedded schema set. |
| `--no-color` | auto | Disable ANSI color in human output. |
| `-q, --quiet` | off | Suppress the per-check summary; print only `RESULT:` and findings. |

---

## 7. "Powered by OMIR" badge gating

The **"Powered by OMIR"** badge signals that an artifact (a published `.omir`
export) or a producer (an implementation that emits OMIR) interoperates at the
guaranteed floor. It is awarded **only** on passing **Core** conformance:

- An artifact is badge-eligible iff `omir-validate <file> --level core` exits `0`
  (`result: "pass"`, `coreConformant: true`, `badgeEligible: true`).
- A producer earns the badge by running the published conformance corpus through
  `omir-validate --level core` in CI with a green result; the JSON reports are
  the evidence the Working Group accepts.

Core is the contract: structural validity against the R1 schemas, fully resolved
intra-Bundle references, and a present, correct `R1` version marker. `strict` and
`profile` are stronger, advisory super-sets — useful for vendors who want to
advertise more, but **not** required for the badge. The badge never depends on
profile conformance, because profiles are proprietary-by-design constraints and
the badge must mean the same thing for everyone.

> Badge issuance policy, the canonical badge SVG, and the trademark/usage terms
> are governed by the OMIR Working Group and live in the governance repo, not in
> this validator. This section specifies only the **technical gate** the badge
> is bound to.

---

## 8. Architecture (intended)

- **Language:** Rust. **License:** Apache-2.0.
- **Schema validation:** a draft-2020-12 JSON Schema engine (e.g. `jsonschema`),
  fed the embedded R1 schema set with local `$ref` resolution (no network).
- **Layered design:** the validation core is a library (`omir-validate` crate)
  exposing `validate(bundle, level, profiles) -> ConformanceReport`; the CLI is a
  thin `clap` front end. This lets producers embed the same conformance logic
  in-process (so an exporter can self-check before writing the badge) and
  guarantees the CLI and embedded checks agree.
- **Determinism:** findings are emitted in a stable order (check group, then
  `entry.index`, then `path`) so reports diff cleanly in CI.
- **No product coupling:** the crate depends on nothing from Veld's BUSL-1.1
  core. It reads files and schemas; that is all.

---

## 9. Out of scope for R1

- Cross-Bundle / federated reference resolution (R1 Bundles are self-contained).
- Semantic validation of memory *content* (truth, usefulness, retrieval quality).
- Repair / auto-fix (a separate `omir-fix` tool may be proposed later).
- Embedding or vector validation (embeddings are extension data, not core in R1).
- Wire-protocol or transport checks (OMIR is at-rest; MCP/A2A own transport).
```