# Toward a Global Standard

> **Non-normative.** This page is a design discussion, not part of the R1 conformance
> surface. Nothing here changes what makes a Bundle valid ([Conformance](./conformance.md)).
> Every change proposed below would enter through the RFC + ballot process and start low
> on the [OMIR Maturity Model](./principles.md#4-honest-maturity-via-omm) — at **OMM-0/1**
> — and earn its level through independent implementation.

OMIR R1 is honest about its origin: its schemas were derived from one production memory
engine, **Veld**. That is a strength — the core is grounded in a system that actually
ships calibrated confidence, decay, Hebbian edges, and tiering — and a risk. A standard
authored by a single implementation is, until proven otherwise, that implementation's
export format with a logo. The work of becoming a *global* standard is the work of
**shedding the assumptions that are true only for Veld** while keeping the cognitive
substance that makes OMIR worth adopting.

This page names those assumptions and proposes the generalizations that would let a
robotics stack, a healthcare agent, a multi-tenant assistant platform, and a coding agent
all read and write the same `.omir` files without loss. It is the OMIR equivalent of
FHIR's long migration from "HL7's resources" to "everyone's resources."

## What already generalizes (keep it)

Before the critique, the parts of R1 that are *not* Veld-specific and should be preserved:

- **Resource + typed-reference model.** Everything is a Resource; links are
  `ResourceType/id`. This is FHIR-proven and domain-neutral.
- **80/20 core + typed `extension[]`.** The escape hatch is exactly what lets the long
  tail of proprietary data travel without bloating the core. See [Extensions](./extensions.md).
- **Honest maturity (OMM).** A promise-about-change per resource type. Domain-neutral.
- **Calibrated confidence as a distribution**, not a bare float ([Memory Semantics](./semantics.md#1-confidence-as-calibrated-belief)).
- **Portable forgetting state** — decay, anchoring, tiers — recorded as *state*, not as a
  mandated algorithm.
- **Bitemporal-ish timestamps** (`eventTime` vs `createdAt`) and temporal invalidation.
- **Two lossless encodings** and the **Profiles** mechanism for domain tightening.

The generalizations below are mostly *additive* — new optional structure and extension
points — precisely so they do not break the parts that already work.

## Generalization themes

Each theme states what is Veld-specific today, why it limits interchange, and a concrete
proposal. The FHIR precedent is cited where one exists, because OMIR is FHIR-modeled and
should borrow FHIR's solved problems rather than reinvent them.

### A. Open vocabularies, not closed enums  *(highest interchange leverage)*

**Today.** `MemoryRecord.kind`, `experienceType`, `Entity.labels`, and `Episode.source`
are **closed enums**, and they lean toward coding/dev-agent life: `code_edit`,
`file_access`, `command`, `prompt`. A robot cannot say "object grasped"; a healthcare
agent cannot say "symptom reported"; a research agent cannot say "hypothesis formed."
`relationType` is already an open string — good — but it has no way to say *which*
vocabulary a term comes from.

**Risk.** Closed enums hard-code one domain's worldview into the core. Every other domain
is forced into `other` + an extension, which means their primary semantics are invisible
to a generic consumer — defeating interchange for everyone but Veld-likes.

**Proposal.** Adopt a FHIR-style **CodeableConcept** for these fields: a structure
carrying an optional `system` (vocabulary URI), a `code`, and human `text`, e.g.

```json
"experienceType": { "system": "https://omir.io/vocab/robotics", "code": "object_grasped", "text": "Picked up the red block" }
```

R1's bare-string/enum forms remain valid as the degenerate case (`text` only). OMIR ships
*core* vocabularies (the current enums, promoted to published code systems) and lets
domains register their own. This single change is the difference between "a memory format
for coding agents" and "a memory format."

### B. Identity, ownership, and multi-agent memory

**Today.** Memory is implicitly **single-agent**. There is no first-class notion of *whose*
memory a record is, who authored it, or how two agents share a memory store. Veld carries
optional `agent_id`/`actor_id` tags, but they are tags, not a model.

**Risk.** The 2026 reality is *fleets* of agents and multi-tenant platforms. Without an
ownership/authorship model, a Bundle exchanged between agents cannot answer "can I trust
this? who wrote it? am I allowed to read it?" — the questions that matter most when
memory crosses a trust boundary.

**Proposal.** Introduce an **`Agent`** (or `Principal`) resource and optional
`authoredBy` / `owner` references on records and episodes; let `Provenance` carry the
chain of agents a memory passed through (see Theme E). Access control itself stays a
profile/extension concern (it is policy, not data shape), but the *identity hooks* the
policy needs belong in the core. Grade `Agent` at OMM-0 and let multi-agent stacks prove
it.

### C. Entity resolution across systems

**Today.** `Entity.id` is bundle-local; there is no canonical external identifier, no
alias set, and no "this entity is the same as that one." Within Veld a UUID suffices
because there is one graph.

**Risk.** Interchange *is* entity resolution. If Acme's "Alice Smith (employee 123)" and
Globex's "@asmith" cannot be declared the same person, two memory stores can never merge —
the whole point of a portable format collapses to per-pair adapters.

**Proposal.** Add FHIR-style **`Entity.identifier[]`** (`{ system, value }` pairs, e.g.
`{ "system": "mailto", "value": "alice@acme.com" }`), an `aliases[]` list of surface
forms, and a `sameAs` link type so a Bundle can assert cross-system identity. This is the
backbone of federation (Theme I).

### D. Beyond text: modality and embeddings

**Today.** `content` is a single `string`. Multimodal data and embeddings are
extension-only. Veld carries image/audio/video vectors internally, but the at-rest core
sees text.

**Risk.** Agent memory in 2026 is increasingly multimodal (vision, audio, sensor). A
text-only core relegates non-text memory to opaque extensions that no generic consumer can
interpret.

**Proposal.** Generalize content to an optional **multi-part content model** —
`contentType` (a media type) plus either inline `content` or a `MediaReference`
(`{ uri, contentType, hash }`) for out-of-line bytes — and define an **algorithm-neutral
`Embedding`** representation (`{ space, model, dims, vector? | ref? }`) that any vendor can
emit without mandating a model. Embeddings stay *optional* (they are derived, not source),
but become *interpretable* rather than vendor-opaque. The **efficiency-bearing** extensions of
this same `Embedding` — Matryoshka prefixes, sparse codes, a temporal-context drift space, and a
VSA structural code — are catalogued separately in
[Efficiency & Information-Bearing Codes](./efficiency.md) (Theme D reframed from "neutral
embeddings" to *efficiency-bearing* codes).

### E. Provenance and trust as a chain

**Today.** `Provenance` is a flat `{ source, sourceType, credibility, externalId }`. Veld's
internal model is richer (a relay `chain` with per-hop credibility and a `verified` flag);
the core flattens it.

**Risk.** When memory crosses vendors, "where did this come from, through whom, and is it
signed?" is a trust-critical question a flat source string cannot answer.

**Proposal.** Align with **W3C PROV**: let `Provenance` carry a `chain[]` of derivation
steps (agent, activity, time, per-hop credibility) and an optional **attestation/signature**
so a consumer can verify a record was not tampered with in transit. Keep the flat form as
the common case.

### F. Privacy, sensitivity, and retention

**Today.** **Absent.** R1 has no notion of data sensitivity, consent, legal basis,
retention/expiry policy, or redaction. `validUntil` is for *contradiction*, not governance.
The `omir-personal-assistant` profile gestures at a PII-class extension, but it is opt-in
and shallow.

**Risk.** A *global* memory format will carry personal and regulated data. Without a
governance vocabulary, OMIR cannot be adopted where GDPR/CCPA/HIPAA-style obligations apply
— which is most of the interesting market.

**Proposal.** Define an optional **governance block** (sensitivity classification, legal
basis, retention policy / `deleteAfter`, redaction markers, consent reference). Likely a
core extension *family* with published URLs first, promoted to core once proven. This is
the theme most likely to decide whether OMIR is adoptable by enterprises at all.

### G. Uncertainty beyond a point estimate

**Today.** `Confidence` is Beta(α, β) + a calibrated point — already better than most. But
it assumes one uncertainty model and conflates evidence count with belief.

**Proposal.** Keep Beta as the recommended default; allow a more general
**uncertainty representation** (credible interval, or a named distribution + parameters) and
optionally distinguish *epistemic* (lack of evidence) from *aleatoric* (inherent
variability) uncertainty for agents that reason about it.

### H. A complete temporal model

**Today.** `eventTime` vs `createdAt` plus `validUntil` / `validAt` / `invalidatedAt` is a
partial bitemporal model, and time is always a precise RFC 3339 instant.

**Proposal.** Make bitemporality explicit and uniform (a *valid-time* interval and a
*transaction-time* interval), and support **imprecise time** ("sometime in 2024", "before
the incident") via interval/precision-qualified instants. Memory is frequently vague about
when; the format should be able to say so.

### I. Federation and cross-Bundle references

**Today.** R1 is deliberately **closed-world**: every reference must resolve *inside* the
Bundle. That is the right call for R1 (it makes conformance decidable), but it blocks
linking a small export to a large shared graph.

**Proposal.** Define an *optional, higher* conformance level for **resolvable external
references** (a reference may target a resource in another, addressable Bundle/store), with
clear rules so the closed-world guarantee remains the default and the validator can still
decide conformance. Federation is how a memory standard scales past one file.

### J. Structured knowledge and typed attributes

**Today.** `Entity.attributes` is `string → string`; `Episode.metadata` likewise. All
structure degrades to stringly-typed key/values.

**Proposal.** Allow **typed attribute values** (number, boolean, dateTime, quantity-with-unit,
CodeableConcept, reference) so a global graph can carry real structured knowledge — a
measurement with units, a date, a link — without serializing everything to strings.

### K. Language and internationalization

**Today.** Text is implicitly English; there are no language tags, and the reference
pipeline's tokenization/NER assume English.

**Proposal.** Add optional **language tags** (BCP 47) on textual content and entity names,
and allow multiple language variants of a `name`/`summary`. A global standard cannot assume
one language.

## A prioritized path

Not all of these are equal. For *interchange* specifically — the existential goal — the
order is clear:

| Priority | Theme | Why first | Likely entry |
|---|---|---|---|
| 1 | **A. Open vocabularies** | Unblocks every non-coding domain at once; small, additive change. | R1.x · OMM-1 |
| 2 | **C. Entity resolution** | Interchange *is* identity; without it, stores can't merge. | R2 · OMM-1 |
| 3 | **F. Privacy / retention** | Gate to enterprise & regulated adoption. | R2 · OMM-0 |
| 4 | **B. Identity / multi-agent** | Matches the 2026 fleet reality; trust across boundaries. | R2 · OMM-0 |
| 5 | **E. Provenance chain + attestation** | Trust when memory crosses vendors. | R2 · OMM-1 |
| 6 | **D. Modality + neutral embeddings** | Multimodal memory becomes interpretable, not opaque. | R2 · OMM-0 |
| — | H, G, J, K, I | Valuable, lower interchange-urgency; fold in as domains demand. | R2+ |

The first item is the highest-leverage and lowest-cost: turning four closed enums into
CodeableConcepts, with the current values published as the seed vocabularies, would by
itself move OMIR from "a coding-agent memory format" to "a memory format that coding agents
use," at OMM-1, without breaking a single R1 document.

## The adoption plan

The prioritization above is a ranking; this is the **plan**. The Working Group's stated
intent is to adopt **all** of the proposals, in a deliberate sequence set by dependency and
leverage — **vocabularies (A) first, identity & resolution (C) next** — with **W3C PROV (E)**
as the trust backbone the rest hangs from, and the refinements that **fix G, J, and K** folded
in once the load-bearing pieces land. Every phase enters at low OMM and is gated by independent
implementation. **"Additive and non-breaking" is a claim with teeth here, not a slogan: every
R1 resource schema sets `additionalProperties: false` (Conformance §Document rules, rule CR-6),
so a new core *field* is rejected by the very R1 schema we promise to keep honoring. We
therefore label each change honestly against GOVERNANCE §5.1/§5.2 and route it through the
correct release vehicle below.**

**A note on the §5.2 boundary, stated once and reused.** *Adding a new, unreferenced `$defs`
member to `common.schema.json` is purely additive under §5.2 (no existing bundle becomes
invalid). It is the **widening of an existing field or def to reference it**, or the widening of
the shared `Reference` pattern, that is non-additive.* This is the principle that lets
`CodeableConcept` (Phase 1) and `TypedValue` (Phase 5) be added cleanly as defs while the union
widening (Phase 1) and the Agent/Reference widening (Phase 2) are correctly breaking. Wherever
the plan calls a shared-schema change "breaking" or "additive," it means this. **A corollary the
plan applies throughout: any field whose value can only be expressed by widening the shared
`Reference` pattern (`authoredBy`, `owner`, an Agent-targeting `sameAs`) is NOT §5.1-additive and
is R2/RFC-gated, full stop — it cannot ride an R1.x increment.**

### Four preconditions before any phase ships

**Each precondition is itself a §3.1 normative RFC** (it amends GOVERNANCE/principles/CONTRIBUTING,
the `common.schema.json` shared defs, or process), so each needs a 14-day-minimum comment window
and a two-thirds TSC ballot (§3.3) **before Phase 1 opens**. Bundle **P0+P0a into one
governance-reconciliation RFC** (they are coupled) and ship **P1** as a second RFC; **P-1 (the
founding-TSC bootstrap) is procedurally first** because no ballot is callable without a seated TSC.

**P-1 — Seat a founding TSC (the true first domino).** GOVERNANCE §2.3 targets a 3–7 seat TSC
filled "by community nomination and confirmation by the sitting TSC" — but **no TSC is seated, and
§2.3 has no bootstrap clause for the *first* one**, so the balloting body is self-perpetuating by
design and cannot start. "Confirm it before P0" is not a mechanism. P-1 (a GOVERNANCE amendment,
in scope) **adds a one-time founding-TSC procedure that does not require a sitting TSC**: a public
call-for-nominations seating an initial **3-seat TSC with at least two non-Veld seats** (so Veld
holds ≤ one-third from day one and the §2.4 cap holds — at 3 seats, one-third rounded down is
zero, so a Veld-majority bootstrap would be unlawful), confirmed by **lazy consensus over a fixed
comment window** rather than by a nonexistent sitting TSC. **P-1 is therefore where external
participation is first recruited, not an administrative formality** — and it couples directly to
P2: you cannot seat a neutral TSC without at least one non-Veld party, the same party the
second-implementer gate needs.

**P0 — Reconcile the OMM ladder *and re-grade the published R1 table*.** The repo carries
**three inconsistent OMM-2 definitions**: GOVERNANCE §4.1 OMM-2 = *"Exercised in real bundles by
at least one implementation"* (GOVERNANCE.md:233); principles.md §4 OMM-2 = *"implemented in more
than one system"* (principles.md:55); and CONTRIBUTING.md §8 **collapses OMM-1 and OMM-2 into one
band** — *"Trial use — implemented somewhere, but limited field experience"* (CONTRIBUTING.md:372-378)
— putting "multiple implementations" only at OMM-3. The reconciliation RFC **MUST rewrite the
ladder identically across all THREE documents** (GOVERNANCE §4.1+§4.3, principles.md §4,
CONTRIBUTING.md §8, splitting its merged "1–2" row), and update the §3.4 Draft/Trial-Use/Normative
band table in the same RFC:
- **OMM-1** = implemented in ≥1 system; field set volatile.
- **OMM-2** = **≥2 *independent* implementations**; shape settling, not yet broad field use.
- **OMM-3** = ≥2 independent implementations **plus** real-world cross-validated usage/conformance
  evidence (the discriminator from OMM-2 is *settling* vs *cross-validated field experience*).

**The reconciliation is monotonic in BOTH directions — it lowers any existing grade the new
independence gate no longer supports.** Today GOVERNANCE §4.3 grades MemoryRecord at **OMM-4** and
Entity/Relationship/Episode at **OMM-3** with Veld as sole implementer (GOVERNANCE.md:255-258),
repeated in principles.md §4 (62-64), CONTRIBUTING.md §8 (382-384), and the README grade table
(README.md:48-51). Under the reconciled gate those grades are unsupportable: there is no second
independent implementation. **The same RFC MUST re-grade the R1 table — demoting MemoryRecord and
Entity/Relationship/Episode to OMM-1** ("settled shape in the reference implementation; awaiting a
second independent implementation") **across all four artifacts in one ballot.** Leaving four
overclaimed grades standing while reforming the OMM in the name of honesty is exactly the
dishonesty §4.2 forbids and the loudest possible "the OMM is marketing" signal to a prospective
second implementer. Until P0 lands, **no phase may claim OMM-2**, and every OMM target below is
read against the reconciled, re-graded ladder. *P0 also assigns **stable rule anchors CR-1..CR-8**
to Conformance §"Document rules (MUST)" (today a positional 1–8 list with no stable IDs), so every
cross-reference in this plan and its RFCs survives renumbering.*

**P0a — Define "independent" so it is recorded, neutral, and not gameable.** An implementation
counts toward the gate **iff** (a) it has a **separate codebase/maintainer**, developed **without
copying reference-implementation code under any license other than the Apache-2.0 grant** —
*re-implementing from the CC-BY spec and using the validator as a conformance oracle explicitly
COUNTS as independent* (otherwise the licensing invariant that anyone may re-implement from the
spec contradicts the gate); (b) it ships under a **separate product/funding line with its own
users**; and (c) it **passes the reference validator while PRODUCING the feature** (not merely
ignoring it). The gate is **decoupled from TSC seats and employer**: a committed second adopter
who *joins* the TSC is exactly what we want and **MUST NOT** be disqualified for it. An
**anti-sock-puppet clause** (common control, shared funding, or shared engineering team → counts
as **one**) replaces any unenforceable string-match on "shared employer." To keep the gate from
being decided by the party that benefits from declaring it met: the **counts-as-one determination
uses the SAME "organization/common control" definition as the §2.4 seat cap** (one bright line
governs both seat math and implementation counting), the TSC **MUST publish written findings
against the three criteria on the ballot record**, and the determination is taken by a
**supermajority EXCLUDING any TSC member employed by or contracted to the reference implementer**
(extending the §2.3 recusal rule — the convening vendor has a material non-interoperability
interest in the gate being declared met). The WG validator and any Veld-authored tool never count.

**P1 — A versioning contract for the R1→R2 line, with a named validator-rewrite scope.** The R1
version pin is enforced at **four** points today; the R2 schema set and the validator MUST handle
each: **(i)** `Bundle.omirVersion const:"R1"` is **neutralized to a plain string in the validator**
(schemas.rs:103) — the literal-R1 envelope gate actually lives in Rust at `version_presence`
(lib.rs:390-402 → E300), so making it version-relative is a **code change, not a schema `const`
edit**; **(ii)** `Meta.omirVersion const:"R1"` (common.schema.json:39 → E301); **(iii)** `Meta` is
`additionalProperties:false` (common.schema.json:58); **(iv)** **every resource schema AND the
Bundle envelope set `additionalProperties:false`** (CR-6; Bundle.schema.json:30). Specify:
1. R2 Bundles **and** resources carry `omirVersion:"R2"`; the R2 `common.schema.json` (under the
   R2 namespace) sets the `Meta.omirVersion` site to `const:"R2"` (or an `enum` of accepted
   majors), leaving R1 schemas frozen. E300/E301 become **version-relative, not literal-R1**.
2. A consumer reads the **major** version and applies that version's schema set, and **MAY**
   reject a newer *major* it does not implement. **Forward-read of unknown *core* fields is
   constrained by closed schemas and is stated honestly:** because R2.0 resource schemas are
   `additionalProperties:false`, an already-shipped R2.0 validator **does NOT silently forward-read
   a new core field introduced by a later R2.x minor** — it would hard-reject it (E120), exactly as
   "R1 readers do not tolerate R2." Intra-major additivity therefore rides the **`extension[]`
   lane** (always tolerated by `ignore-unknown-extensions`), **not** undeclared new top-level
   properties. Any genuinely new *optional core field* that we want same-major consumers to skip is
   shipped only when a minor schema bump relaxes the relevant object to a **controlled, pattern-
   gated extension lane**; absent that, it is a new-release item. The earlier blanket
   "SHOULD ignore unknown newer-minor fields" claim is dropped as unimplementable against closed
   schemas — this corrects the Overlook table's "R1.x-additive ⇒ forward-readable" assumption.
3. **Forward-read across majors is a property of R2+ tools about older majors, never of
   already-shipped R1 tools about newer ones.** A published R1 validator **WILL** reject any R2
   bundle at the envelope (E300) **by design**. The honest compatibility guarantee is exactly
   *"R1 bundles stay valid forever"* + *"R2 readers accept R1,"* **not** *"R1 readers tolerate R2."*
4. The reference validator becomes **version-aware** as a named, scoped rewrite, not a config
   switch: **(a)** parameterize `SchemaFiles`, `COMMON_ID`, and `RESOURCE_TYPES` (schemas.rs) by
   declared major version (an R1 set and an R2 set, each with its own `common.schema.json` `$id`);
   **(b)** build a version-keyed `Registry` and dispatch in `Schemas::build`; **(c)** the validator
   **reads the declared Bundle-level `omirVersion` at PARSE time, before any structural / reference
   / version check, and selects the version-keyed registry; E110/E120/E200/E300/E301 then all run
   against the selected set.** (Selecting "before E300" is insufficient: today structural E110/E120
   run *first* (lib.rs:31-132) and E300 last (lib.rs:142), so an R2 bundle would be double-reported
   as wrong-version AND malformed-against-R1.) An unknown newer major **short-circuits to a single
   E3xx at the envelope and runs NO resource-schema checks**; **(d)** the embedded-schema
   `include_str!` mechanism carries both releases' schemas. **DoD:** P1 is done only when the
   **same binary** validates a published R1 example AND an R2 example, and reports an R2-declared
   bundle as a single envelope-level E300 under the R1 set.

### The phases

**Phase 1 — Vocabulary (A).** **Split into 1a (the cheap adoption surface) and 1b (closed-vocabulary
enforcement); 1b does NOT gate 1a and is not on the second-implementer critical path.**

*Phase 1a, in two decoupled moves so namespace-opening costs nothing on day one.* **(1) Open the
namespace first, additively.** Publish the seed code systems and the vocabulary registry under
**`https://omir.io/spec/R2/vocab/<field>`** as an **R1.x-additive documentation + JSON-LD artifact**
(no schema type change), so other domains can NAME their own concepts immediately with **zero
bundle becoming R2**. This is the real payoff of A-first under the directive — *opening the
namespace*, not paying a major-version envelope break before any consumer benefits. **(2) Widen the
union when a coded value is demanded.** Add a **`CodeableConcept`** def (`{ system, code, text }`)
to `common.schema.json` (purely-additive per the §5.2 note) and widen `kind`, `experienceType`,
`Entity.labels`, `Episode.source`, **and `Relationship.relationType`** (the spec's one *already-open*
vocabulary — CONTRIBUTING §3.10:137-140, GOVERNANCE §5.1:282 — and the single largest real-world
fragmentation vector; widening it is the *least* breaking because its bare branch is already
`type:string`). **Widen each closed enum's string branch to the EXISTING enum, not to `type:string`:**
`anyOf: [ <the R1 enum>, CodeableConcept ]` (for `relationType`, `anyOf: [ {type:string},
CodeableConcept ]`). This keeps the legacy closed set hard-validated at **Core** (a bad `kind`
like `"banana"` still fails E120). **Make the seed/object mapping STRUCTURAL, not prose:** in the
R2 schema, constrain the `CodeableConcept` branch of each widened field so its `system` **MUST NOT
be that field's own seed-system URL** (`not:{properties:{system:{const:<seedURL>}}}`), so a seed
value is expressible **only** as the bare string and the union is genuinely bijective at the Core
schema layer — no Phase-1b dependency, no honor-system. Move (2) **ships as the first R2 change**;
move (1) does not. Adding or retiring a seed value is governed per the operation, below.

*Seed-registry governance, split by direction (so additive growth stays cheap):*
- **ADDING a seed value** to an existing seed system is **§5.1-additive open-vocabulary growth**
  (it matches GOVERNANCE §5.1's existing "new open-vocabulary values" carve-out): an ordinary PR
  under Maintainer review, **NOT** a full RFC. Routing every new concept through a 14-day ballot
  would make A *heavier* than today's open-vocab path — the opposite of cheap adoption.
- **RETIRING, narrowing, or re-meaning a seed value** is enum-narrowing: **§3.1 RFC + ballot**, and
  (see Phase 3) a **canonicalization-version bump under §6** so historical signatures are verified
  under the frozen seed table of their own version, never retroactively flipped.

*Normalization, stated precisely and per-field cardinality:*
- A bare seed string `s` and the seeded coding `{system:<seed URL>, code:s}` are **semantically
  equivalent for matching/comparison**; the **bare-string seed form is the canonical at-rest form**.
  **R2 introduces a NEW consumer rule** (not a restatement of the existing extensions-scoped SHOULD):
  an **R2 consumer MUST NOT canonicalize between bare-string and object form for seed-coded fields
  on re-export** — lossless pass-through. This is a consumer-side tightening listed in the R2
  migration note; equality is a consumer obligation, not a structural check (the core validates
  union **shape** only, with the seed-system exclusion above making the shape itself bijective).
- For **fields carrying a schema `default`** (`kind`, `tier`): **do NOT mutate the R1 `default` in
  place** (MemoryRecord.kind:18 `default:"memory"`, tier:31 `default:"working"` live on an OMM-1
  type and are read by default-applying codegen/normalizers). The R2 schemas **keep `default`**;
  the **canonical at-rest form is *omission* when a field equals its default** (not the explicit
  value). Phase 3's canonicalizer normalizes a defaulted field to its **omitted** form before
  building the signed map M, so absent and explicit-default sign identically. Removing/relocating a
  `default` would change the value seen by default-applying consumers — so this is a **documented
  R2 migration item under "behavior of absent fields"** for **both `kind` and `tier`** ("an absent
  `kind` MUST be interpreted as `memory`; an absent `tier` as `working`"), not a silent edit.
- For **scalar optional fields with no default** (`experienceType`, `Episode.source`): **absent
  stays absent** and is distinct from `{text:s}`.
- For the **array field** `Entity.labels`: the legacy `"other"` + label-extension idiom is
  **DEPRECATED** under §6 in favor of a CodeableConcept item. Update `context.jsonld`
  `labels`/term mappings accordingly.
- A **non-seed** coded value **MUST** use the object form; a **seed** value **MUST** use the bare
  string — now enforced structurally (above), so Phase-3 attestation canonicalization collapses the
  union to one byte sequence.

***Companion item (atomic with the widening, NOT deferred to 1b): re-key every behavioral MUST that
references a widened enum value.*** profiles.md:88-90 makes prospective-memory filtering a
behavioral MUST keyed on the literal `experienceType:"intention"`. The moment 1a admits the object
form, an unmodified profile MUST is silent on `experienceType:{system:<seed>, code:"intention"}` —
a safety-relevant recall gap. The **normative re-key** ("any `experienceType` whose canonical form
is the seeded `intention` coding MUST be filtered from ordinary recall") **lands in the SAME RFC as
the experienceType widening**, with a 1a-DoD fixture proving an object-form `intention` is still
filtered. Only the machine-readable binding *engine* stays in 1b.

*JSON-LD `@context` audit (a DoD blocker, not an afterthought):* widening a field that shares an
overloaded `@context` term silently corrupts the RDF lift of existing bundles. In R1, **one term
`source` is bound to a single bare `omir:source` with no per-field `@type` (context.jsonld:33), yet
four fields use it**: `Episode.source` (being widened), `Bundle.source`, `Meta.source`,
`Provenance.source` (all plain strings). A coded `Episode.source` would lift a `{system,code,text}`
node onto a predicate that elsewhere carries a literal, and `system`/`code`/`text` (unmapped) would
fall to accidental `@vocab` predicates. **Before widening any field, split every overloaded term in
the R2 `context.jsonld`:** give the widened `Episode.source` its own term (`episodeSource →
omir:episodeSource`) typed for an embedded coding, add `CodeableConcept`'s `system`/`code`/`text`
with explicit typing, and keep the three string `source` fields on a separate literal predicate.
**DoD:** a Turtle/N-Quads golden-file test that the R1 minimal bundle and an R2 coded bundle both
lift to the intended triples.

*Honest status:* this is a **type change to existing fields**, two of which live on MemoryRecord
(now OMM-1 under P0). A bare-string producer stays valid; an object value is rejected by an R1
consumer; an R2 bundle is opaque to an unmodified R1 consumer at the envelope (per P1). *Domain
vocabularies (robotics, healthcare) live under their own non-omir.io URLs; the omir.io namespace is
reserved for WG-published seed systems only.* Target: **R2 · OMM-1**. *A-first is about **opening the
namespace** (move 1, day-one, additive) so other domains can name their own concepts; the union
widening (move 2) is held to R2 and is the cheapest *additive spec change* — but milestone zero
(below) is the literally cheapest adoption.*

*Phase 1b — closed-vocabulary enforcement (separate work item, off the 1a critical path).*
**Extend profiles.md** with a **machine-readable** terminology-binding artifact (a profile schema,
not prose) binding a CodeableConcept field to a code system/value set with a binding strength
(`required`/`extensible`/`preferred`), mirroring FHIR. **Implement profile-constraint checking** in
the reference validator — today there is **no code path** (lib.rs:144 is a bare comment): this is a
**net-new subsystem** (profile loader, `meta.profile` dispatch, value-set membership check, new
finding codes: `required` → error, `extensible`/`preferred` → warning), with passing and failing
examples. **Re-issue the three R1 reference profiles**, splitting by kind:
- **Genuinely REQUIRED enums** (`omir-coding-agent.kind`) → `required`-strength bindings. **The
  required-strength binding requires the field to be PRESENT** — an absent `kind` relying on the
  Phase-1a normative default does **not** satisfy the profile (a profile narrows the base; presence
  is the thing it guarantees). Cross-referenced from the 1a "default" bullet.
- **SHOULD value fields** (`omir-coding-agent provenance.sourceType` (profiles.md:41),
  `Entity.labels`) → `preferred`/`extensible` bindings reported as **warnings, never errors**.
  **Binding strength MUST match the original RFC-2119 keyword; promoting a SHOULD to an enforced
  `required` binding is a new-release narrowing, not a re-expression.**
- **Behavioral MUSTs** are **NOT terminology bindings** and stay as prose RFC-2119 requirements.
  Re-issuing a profile **MUST preserve every behavioral MUST verbatim** (the intention-filtering
  re-key already landed in 1a); terminology bindings constrain **values**, not consumer behavior.

**Phase 2 — Identity (B: Agent) and Resolution (C).** **B (Agent) is a prerequisite ONLY for E2
(agent-attribution hops); the headline trust deliverable E1 depends only on A and the existing
`Provenance` and ships without Agent** — so Agent's adoption gate never holds the trust pillar
hostage. **B:** introduce an `Agent` (Principal) resource and optional `authoredBy` / `owner`
references. **C:** add optional `Entity.identifier[]` (`{ system, value }`), `aliases[]`, and a
`sameAs` link so two stores can declare cross-system identity and merge.
*Honest status, partitioned by whether a field touches the `Reference` pattern:*
- **§5.1-additive (R1.x-eligible):** `Entity.identifier[]` and `aliases[]` (a new non-`ref` def
  `{system,value}` and string array — no `Reference` widening).
- **§5.2/§3.1-breaking (R2/RFC-gated):** **any field that can reference `Agent`** — `authoredBy`,
  `owner`, and `sameAs` insofar as it permits Agent targets — because each forces the
  `common.schema.json#/$defs/Reference.pattern` widening. These are **NOT** R1.x-additive.
Adding `Agent` does not invalidate any existing R1 bundle, but the conformance/bundle prose
enumerating "the four core resources" **must** change in the same PR. Required, atomic work items:
- **(a1)** author `schemas/Agent.schema.json`; **(a2)** add its `$ref` to `Bundle.entry.items.oneOf`
  (a `oneOf` of resource-schema `$ref`s, **not** a regex); **(a3)** widen the
  **`common.schema.json#/$defs/Reference.pattern` regex** (today `^(MemoryRecord|Entity|
  Relationship|Episode)/...`, common.schema.json:18) to include `Agent` (the single load-bearing
  pattern change).
- **(a4)** extend the validator's `SchemaFiles` struct + `from_dir`/`embedded` loaders + `by_type`
  map (schemas.rs is a hard-coded 6-file / 4-type shape that will not pick up a new type
  automatically); update `RESOURCE_TYPES` (item b) and **both** "four core resources" message sites:
  the validator E101 string (lib.rs:124) **and** validator/README.md:93.
- **(b)** add `Agent` to `RESOURCE_TYPES`.
- **(c)** **replace the hand-coded per-field reference walk with a registry-driven walker.** The
  mandate is **not** "resolve every `{ref:"Type/id"}` anywhere." It resolves **(i)** every
  Reference-object `{ref:...}` at a **schema-declared Reference-typed field** — **NEVER** inside
  `extension[].valueJson` or any free-form/`additionalProperties` map (a vendor may legitimately
  put a `{"ref":...}`-shaped object in `valueJson`); **and (ii)** every closed-world **bare-id**
  field enumerated in CR-5 (currently `MemoryRecord.parentId`, special-cased at lib.rs:219-230).
  Drive the walk from an **explicit registry of `(field-path → target-type-SET, ref-shape)`
  entries** (a **set**, because some R2 slots are poly-typed — see (c′)). **Seed the registry with
  the FULL existing closed-world set as ground truth** from lib.rs:217-239 and CR-5: `entityRefs →
  {Entity}` (on MemoryRecord and Episode), `Relationship.from → {Entity}`, `Relationship.to →
  {Entity}`, **`Relationship.sourceEpisode → {Episode}`** (silently dropped in earlier drafts — its
  E201 expected-type=Episode check must survive the refactor), `MemoryRecord.parentId →
  {MemoryRecord}` (bare-id). Adding `authoredBy`/`owner`/`sameAs`/`provenance.chain[]` then
  **extends the registry** rather than relying on shape-sniffing, and the per-field **target type is
  preserved** (E201 must still reject, e.g., `Agent/x` in `Relationship.from`).
- **(c′)** Change E201's check from `ref_type == expected` to `expected_set.contains(ref_type)`
  (lib.rs:337), since Phase 3's `provenance.chain[].from` is **poly-typed** (any in-Bundle resource)
  while `chain[].agent` is **mono-typed** to `{Agent}`. The current single-type tuple cannot model
  this; the set form can.
- **(d)** amend Conformance CR-5 by **appending** the new reference-bearing fields to the existing
  closed-world list (never re-deriving it from the Reference type); state that the closed-world set
  is exactly the schema's Reference-typed fields **plus** the enumerated bare-id fields, and record
  the **per-field target-type table** in the migration note.
- **(e)** register `Agent` in the JSON-LD `@context`; **(f)** ship a migration note.

For Phase 5's federation form, **do NOT relax the shared Reference pattern**: define the
external-reference form as a **distinct `$def` (`ExternalReference`, its own property, not `ref`)**
so the Core closed-world pattern is untouched. Target: **R2 · B (Agent): OMM-0 / C: OMM-0** —
C is graded **OMM-0 "mechanism present, cross-system merge unverifiable single-party"**, not OMM-1:
`identifier[]`/`aliases`/`sameAs` are inert until either a second store exists or federation (I)
lands. C's identifier fields and I's federation mechanism are a **matched pair** whose combined
value requires the second implementer; neither is claimed as delivered interop value at single-party.

**Phase 3 — Provenance & trust: W3C PROV (E), the pillar.** Re-shape `Provenance` into a
PROV-aligned derivation chain with optional, redaction-aware attestation. Detailed below. E splits:
**E1** (chain over prior resources + per-hop credibility) depends only on A and the existing
`Provenance`; **E2** (agent-attribution hops referencing `Agent`) depends on B. **The critical
path is `A → E1-chain`** — pure additive optional structure, immediately reviewable by a second
party. **The heavyweight attestation subsystem (det-CBOR / M / inclusion allow-list / redaction
commitments / key resolution) is a PARALLEL track explicitly OFF the critical path**, because
cross-verification is its only meaningful test and is worthless with one implementer: it cannot
reach OMM-1 or interop until the P2 second-verifier milestone is met. This honors "E emphasized" as
the trust **design** pillar without front-loading a multi-quarter cryptographic interop project
ahead of any second implementer. **The E1/E2 discriminator is normative:** a record is **E1** only
if its chain contains **no `agent` operand and no agent-naming attestation**; the moment any hop
carries `agent` **or** the attestation's signed subset includes an `Agent` reference, it is **E2**
and capped at the Agent OMM floor. E2 **MUST NOT** advance past OMM-0 until `Agent` is at least
OMM-1 with Phase-2 Reference support. Target: **R2 · E1-chain: OMM-1 / attestation & E2: OMM-0
(capped at the Agent floor; non-interoperable until P2)**.

**Phase 4 — Sensitivity & modality (F, D).** **F:** an optional governance block (classification,
legal basis, retention / `deleteAfter`, **redaction markers**, consent reference) — plus a typed
**`redactionCommitments[]`** array (`{ fieldPath, commitment, salt }`) so per-field signed
commitments have a declared schema home under `additionalProperties:false` (without it there is no
legal place to store a commitment for an arbitrary redactable field). The redaction *mechanism* is
co-designed with E's attestation envelope (Phase 3). On introducing the core governance block,
**deprecate the WG-published, omir.io-namespaced, profile-REQUIRED `pii-class` extension**
(`https://omir.io/spec/R1/extension/pii-class`, profiles.md:91-93) under §6 (deprecated R2, earliest
removal R3): because it is **required** by `omir-personal-assistant`, during the R2 window that
profile **MUST accept EITHER the deprecated extension OR the core `classification` field** (so no
existing producer is instantly non-conformant), and the R2 migration note **MUST carry an explicit
field-by-field value-shape mapping** (pii-class → core `classification`). **D:** a multi-part
content model (`contentType` + inline-or-`MediaReference`) and an algorithm-neutral `Embedding`
representation `{ space, model, dims, dtype, vector?|ref? }` where `space` is an **opaque equality
key** (vectors comparable **iff** `space` strings are byte-equal; no cross-`space` comparability
claim), `dims` **MUST** equal the vector length, and `dtype` pins the byte form. *On-the-wire:* a
JSON decimal array in `.omir`, a typed CBOR array in `.omirb`. **Because `.omir` decimal and
`.omirb` dtype bytes are not byte-reconstructable from each other, embedding vectors are EXCLUDED
from signed images by default; if ever committed, the commitment is over a single dtype-independent
canonical-decimal (shortest-round-trip) form, never the dtype byte form** (see Phase 3). *Honest
status:* F and D are **new optional fields**; absent the Agent/ref dependency they qualify for
R1.x-additive (footnote ¹). Target: **R2 · OMM-0** (D grades OMM-0 until two vendors demonstrate a
cross-store similarity round-trip).

**Phase 5 — Refinements: fix G, J, and K (and fold in H, I).**
- **G — uncertainty:** keep Beta as default; **add** a general uncertainty value (credible
  interval, or named distribution + params) and optionally separate *epistemic* from *aleatoric*.
  New optional fields — R1.x-additive-eligible.
- **J — typed attributes:** **do not replace** the string maps. **Add** an additive optional
  sibling channel — `Entity.typedAttributes[]` / `Episode.typedMetadata[]`, each `{ key, value }`
  where `value` is a **shared `TypedValue` def in `common.schema.json`** (number, boolean,
  dateTime, quantity-with-unit, CodeableConcept, reference; the def-add is purely additive) — and
  **mark `attributes`/`metadata` DEPRECATED** under §6 (deprecated R2, earliest removal R3),
  updating `context.jsonld`. **Adding the typed channel AND marking the maps deprecated are both
  §5.1-additive and R1.x-eligible** (per §6.4 validators only *warn* on deprecated-but-present
  items); **only the eventual R3 REMOVAL of the maps is the breaking boundary.** CodeableConcept
  (from A) is one member of the union; keep the dependency `A → J`, and J's typed channel caps at
  A's grade. **`TypedValue` is NOT reused in the `Extension` value slots** — constraining them to a
  closed union would remove arbitrary `valueJson` (a §5.2 break violating the typed-extension
  escape-hatch invariant). `Extension.valueJson` stays maximally permissive; if typed extension
  values are later wanted, **add** an optional `valueTyped` (`$ref TypedValue`) **alongside** the
  existing `value*` fields (purely additive) — never retro-type `valueJson`.
- **K — i18n:** BCP-47 **language tags** on textual content and entity names, with optional
  multilingual variants. New optional fields — R1.x-additive-eligible.
- Fold in **H** (full bitemporality + imprecise/interval time — new optional fields) and **I**
  (federation). **I requires a mechanism before a target:** introduce the syntactically distinct
  **`ExternalReference` `$def`** (its own property, an absolute/identifier-based locator built on
  Theme C's identifiers, **never** the bare `ResourceType/id` and **never** a relaxed Core pattern);
  define a third conformance level **"Federated"** in Conformance.md (via RFC) that relaxes CR-5
  **only** for explicitly-external references; **closed-world remains the Core default and the
  R1/R2 invariant is preserved** for anything not claiming Federated. **I is blocked by (1)** the
  widened Reference (Phase 2 B-work), **(2)** the new `ExternalReference` `$def`, and **(3)** the
  CR-5 carve-out RFC — **C's identifiers are an *input* to how external references are expressed,
  not the gate.** Target: **R2+ · OMM-0**.

### Phase 3 in depth — aligning `Provenance` with W3C PROV

W3C **PROV** models the world with **Entity** (a thing), **Activity** (something that acts over
time), and **Agent** (who/what is responsible), related by a **type-constrained** verb set whose
relations take incompatible operands: `wasAttributedTo` is Entity→Agent; `wasDerivedFrom` is
Entity→Entity; `wasGeneratedBy` is Entity→Activity (generated entity is the subject); **`used` is
Activity→Entity** and **`wasInformedBy` is Activity→Activity** — both Activity-subject. OMIR's flat
`{ source, sourceType, credibility, externalId }` is the *common case*; the proposal lets it
**expand into a chain** when trust matters. **The chain's implicit subject is the resource carrying
the `provenance` block, lifting to `prov:Entity`.** Each step is a **discriminated union keyed on
`role`** that pins its legal operands. **To avoid a fatal `@context` term collision, the chain does
NOT reuse the globally-bound terms `from` (Relationship operand, context.jsonld:158) or
`credibility` (context.jsonld:91): it uses distinct field names `derivedFrom` and `hopCredibility`**
(JSON-LD term definitions are document-global, not subtree-scoped — the same term cannot mean two
things under one merged context).

```json
"provenance": {
  "source": "design-review",
  "credibility": 0.92,
  "aggregateCredibility": { "value": 0.75, "model": "product" },
  "chain": [
    { "role": "wasAttributedTo", "agent": { "ref": "Agent/agent-a" },         "at": "2026-05-30T11:42:00Z", "hopCredibility": 0.95 },
    { "role": "wasDerivedFrom",  "derivedFrom": { "ref": "Episode/ep-launch-chat" }, "at": "2026-05-30T11:42:03Z", "hopCredibility": 0.90 },
    { "role": "wasGeneratedBy",  "activity": { "id": "act-consolidation", "label": "consolidation", "at": "2026-05-31T03:00:00Z" }, "hopCredibility": 0.88 }
  ],
  "attestation": { "alg": "ed25519", "keyId": "did:web:example.org#k1", "key": "<inline JWK>", "agent": "Agent/agent-a", "at": "2026-05-31T03:00:01Z", "signature": "base64url…" }
}
```

- **Role/operand validation is normative and enforced.** With the bearing resource as subject, the
  **Entity-subject** verbs are well-formed and enumerated: `wasAttributedTo` → MUST carry `agent`,
  MUST NOT carry `derivedFrom`; `wasDerivedFrom` → MUST carry `derivedFrom` (a prior resource), MUST
  NOT carry `agent`; `wasGeneratedBy` → MUST carry an `activity` operand, MUST NOT carry `agent`. A
  validator rule **rejects role/operand mismatches.** The **Activity-subject** verbs `used`,
  `wasAssociatedWith`, **and `wasInformedBy`** are **ALL reserved** (none can take the bearing
  `prov:Entity` as subject) until a first-class Activity referent exists. (`wasInformedBy` is
  Activity→Activity, the same subject-type problem as the others — it was previously and
  contradictorily permitted with an inline activity; that permission is **dropped**, leaving
  `wasGeneratedBy` as the only Activity-touching verb in R2.)
- **`derivedFrom` is poly-typed; `agent` is mono-typed.** `wasDerivedFrom.derivedFrom` may resolve
  to **any in-Bundle resource type** (Episode, MemoryRecord, Entity, Relationship, Agent), so its
  registry entry carries the **full resource-type set** and role/operand validation is what narrows
  it per hop; `wasAttributedTo.agent` is mono-typed to `{Agent}`. The widened E201 set-membership
  check (Phase 2 c′) covers both.
- **Activity is an inline shape, not a literal and not a top-level resource.** R2 carries Activity
  **within the hop** as `activity: { id, label?, at? }` (no new `Bundle.entry`/oneOf type, so the
  "five core resources" prose is untouched). The PROV-O lift mints a **blank-node `prov:Activity`**
  from it. Activity operands carry **no `ref`** and are **EXEMPT from closed-world resolution**; the
  walker keys strictly on `ref`/`derivedFrom`/`agent` and skips them. Promoting Activity to a
  first-class resource is deferred.
- **Closed-world applies to the resource-typed chain refs.** `chain[].agent` and
  `chain[].derivedFrom` are **closed-world references in R2** and MUST resolve within the Bundle;
  the registry-driven walker covers them. **Chain-ref integrity is a DOCUMENT property:** the
  validator walks it unconditionally. A producer that cannot include an upstream Agent/Episode MUST
  inline a minimal resource, satisfy the hop with a **placeholder tombstone** of the right type, or
  omit the hop — never emit a dangling ref. A placeholder tombstone Agent MUST carry an explicit
  `placeholder:true` marker. **Cross-Bundle provenance is deferred to Federated**, never permitted
  at Core; until I lands, attested records carry their provenance closure.
- **Per-hop credibility is the only normative trust number; any roll-up is optional and
  non-normative.** `chain[].hopCredibility` is a **UnitInterval**, added to the CR-7 [0,1]
  enumeration and bound to `common.schema.json#/$defs/UnitInterval`. The legacy
  `provenance.credibility` **keeps its R1 meaning** — it is **not** silently redefined as a derived
  product. Any roll-up lives in a **new optional** `provenance.aggregateCredibility { value:
  UnitInterval, model: "product"|"min"|… }`. **`product` is a series-reliability / weakest-chain
  heuristic, NOT an independent-evidence probability** — the "independent-evidence" label is dropped
  as false. A SHOULD-level check warns when `aggregateCredibility.value` is inconsistent with
  `model` over the present hops (rounding-tolerant).

- **Attestation — explicitly an OMM-0, off-critical-path, parallel track** until two independent
  verifiers demonstrate cross-encoding verification AND the A-normalization and J-migration
  canonicalization versions are pinned. **The validator cannot enforce any of this today** (Outcome
  is a closed `{Pass, Fail}` enum, report.rs:64-69; `Check` is `{Structural, ReferenceIntegrity,
  VersionPresence, Profile}`, report.rs:45-52; lib.rs has no crypto, no CBOR, and reads only parsed
  `.omir` JSON). The attestation track therefore includes an **honest validator work item paralleling
  Phase 1b's "net-new subsystem":** **(a)** add an `.omirb`/CBOR reader with a declared sub-profile
  tag byte; **(b)** add a **`Check::Attestation`** variant and a per-attestation finding triplet
  **`verified` / `tampered` / `unverifiable`** that is **SEPARATE from the document-level Pass/Fail**
  — an `unverifiable` attestation **MUST NOT** flip `core_conformant` to false (it is orthogonal to
  schema conformance); **(c)** scope the ed25519/key code as OMM-0 and **OUT of the Core conformance
  path**, so a Core-R2 consumer that does not verify is still conformant. **Until (a)-(c) land, the
  bincode-under-attestation rule is documentation only and attestation is OMM-0/non-interoperable —
  no validator-enforced MUST is claimed.** The mechanism, when built:
  - **Signing input is a canonical typed map M** defined by a **closed, version-tagged INCLUSION
    allow-list**, enumerated **per signable resource type**. For **MemoryRecord**, M includes `id`,
    `resourceType`, a `content`-commitment, the **ordered** chain (roles, operand
    `ref`/inline-activity-id, per-hop `hopCredibility`), the asserting `Agent` ref + `keyId` + the
    **inline signing key** + signing-time `at`, and `classification` once Phase 4 lands; M
    **excludes** mutable/operational fields (`decay.*`, `meta.lastUpdated`, `version`). Entity/
    Relationship subsets, if signable, are enumerated separately (Entity excludes `mentionCount`,
    `salience`, `lastSeenAt`; Relationship excludes `strength`, `validAt`, `invalidatedAt`).
  - **Scope.** An attestation signs the bearing resource's enumerated subset **plus a
    hash-commitment of each referenced resource's `id`+`resourceType`** (not their mutable bodies).
  - **One canonical byte form.** The signature is over **`SHA-256(det-CBOR(M))`** (RFC 8949 §4.2).
    A JSON (`.omir`) signer/verifier **MUST construct the identical typed map M and the identical
    det-CBOR(M) bytes**; JCS is at most an aid to building M, never an alternate signed form. **No
    field whose `.omir` and `.omirb` representations are not provably byte-reconstructable from each
    other may enter a signed commitment** (dtype-pinned binary byte forms MUST NOT) — this clause is
    also written into encodings.md so encoding-neutrality (Principle 5) and one-canonical-form hold
    simultaneously.
  - **Number normalization is a single pre-CBOR step** applying to **ALL** signed numeric fields
    (not only UnitInterval/score): every JSON number in M maps to its **shortest round-tripping
    decimal**, so `9`, `9.0`, `9.00` collapse (the shipped corpus already drifts —
    `{alpha:9,beta:1}` in encodings.md:36 vs `{alpha:9.0,beta:1.0}` in minimal-bundle.omir:82) and
    **CBOR floats are forbidden for signed numeric fields**. A KAT fixture proves `{alpha:9}` and
    `{alpha:9.0}` sign identically. Defaulted fields are normalized to their **omitted** form before
    M is built, so absent and explicit-default sign identically (Phase 1a). A KAT fixture proves two
    MemoryRecords differing only in presence/absence of a default-valued `kind` produce the identical
    digest.
  - **CodeableConcept-union fields are pre-canonicalized into M** (seed values → bare string —
    enforced bijective by the Phase-1a seed-system exclusion — non-seed → `{system,code}` with
    `text` dropped) **before** CBOR. The **canonicalization-profile version is carried INSIDE M**
    (not merely on the envelope), and the verifier selects canonicalization rules by the version
    recorded **in M**, never by its current profile. A seed retirement (Phase 1a) is a
    canonicalization-version bump; attestations are verified under their own version's **frozen,
    per-release-published seed table (§5.3)** — so seed evolution is **non-retroactive** to
    historical signatures.
  - **Ordered chain, not a set.** M commits to the **ordered** chain (a det-CBOR array of per-hop
    commitments, equivalently a Merkle root), so reorder/drop/duplicate is detected.
  - **Key authority is self-contained — verification is decidable from the Bundle alone.** Because
    OMIR is an **at-rest** format with a closed-world invariant, a `verified` result **MUST be a
    pure function of the bundle bytes**. M therefore **inlines the full public key (JWK)** used and
    **MAY** inline a short Agent-signed key-authorization assertion binding `keyId` at `at`;
    verification = signature valid over M + key-binding valid + `at` within validity. **Out-of-band
    DID/HTTPS key resolution is demoted to an OPTIONAL Federated-level enhancement** (it belongs
    with Theme I, which already gates open-world). A `sameAs` merge **MUST NOT rewrite a signed Agent
    id**. Lawful key rotation/revocation does not retroactively flip historical attestations.
  - **Redaction is cryptographically honest — and the security claim is stated honestly.** For each
    redactable field M contains a **`commitment = H(field-bytes ∥ per-field-salt)`** (stored in the
    Phase-4 `redactionCommitments[]`) and **MUST NOT** contain the plaintext. Redaction deletes only
    the plaintext and leaves the `commitment`, so the signature still verifies; absent plaintext +
    intact signature-covered commitment = **"lawfully redacted, chain intact," NOT tampered**.
    **Because the salt is retained in the at-rest document, it provides ZERO brute-force resistance
    for low-entropy fields** — the honest property is **unlinkability** (commitments are
    non-correlatable across resources/documents), not brute-force resistance. For genuine
    brute-force resistance a producer MUST use a high-entropy per-field nonce that is itself
    **deleted at redaction time** (accepting that such fields can never be re-verified against
    plaintext) **or** explicit out-of-band salt escrow; the chosen model is recorded. The
    impossible "tombstone bearing the same hash" phrasing is dropped: the tombstone bears a
    `redacted:true` marker; the sibling `commitment` carries the hash.
  - **Erasure dominates the trust pillar — the conflict is acknowledged, not legislated away.** When
    a data subject's erasure is legally compelled and the subject *is* the signing Agent, retaining
    id + key-commitment + an attributing signature can itself be unlawful personal data. So: Agent
    identity enters M via the **erasable salted commitment** (above), not a raw resolvable ref;
    erasing the Agent's PII deletes the plaintext while the commitment + signature survive, and
    closed-world is preserved by a **placeholder tombstone carrying that commitment**. The earlier
    "MUST NOT sign over a placeholder Agent reference" is relaxed to **"MUST NOT sign over a
    placeholder that carries no key-commitment"** — i.e. sign over the commitment, not the
    resolvable id. When erasure nonetheless forces signature invalidation, the verifier reports the
    record as **`unverifiable-by-erasure`** (a defined sub-outcome), **NOT** `tampered`. The hard
    privacy invariant is **not** subordinated to the trust pillar.
  - **Encoding cannot silently break a signature — and the rule lives in the SPEC, not only the
    validator.** A producer emitting `.omirb` for a bundle containing any `provenance.attestation`
    **MUST** use the **CBOR sub-profile**, never **bincode** (bincode is not self-describing and
    cannot reconstruct M). This narrows an existing release-published allowance, so it is an **R2
    normative edit to encodings.md §".omirb binary profile" (encodings.md:84-101) under §3.1/§5.2**:
    amend the "bincode permitted as an internal sub-profile" sentence (encodings.md:90) to
    "…permitted **EXCEPT when the bundle carries any `provenance.attestation`, in which case the CBOR
    sub-profile is REQUIRED**." The validator `Check::Attestation` then enforces a rule the spec
    actually states. An attestation whose canonical form cannot be reconstructed is reported
    `unverifiable` — a third outcome distinct from `verified`/`tampered`. A cross-verifier
    **known-answer test vector** (canonical det-CBOR bytes + digest over the worked example) ships
    as a DoD artifact, gated on the P2 second verifier.
- **The PROV-O lift is a SEPARATE opt-in context that does NOT silently compose under `@vocab`.**
  The base `context.jsonld` sets `@vocab: https://omir.io/ns#` (line 4) and `resourceType → @type`,
  so unmapped chain terms would otherwise mint accidental `omir:role`/`omir:at` predicates and the
  chain hops (which carry **no `resourceType`**) have nothing for `@type` to attach to. The
  **`https://omir.io/spec/R2/prov-context.jsonld`** therefore: **(a)** re-declares the `provenance`
  term, dropping the inherited `@type:@id` (context.jsonld:144) and declaring it an embedded node
  with typed interior `chain`/`agent`/`derivedFrom`/`activity` terms; **(b)** sets `@vocab: null`
  within the chain/attestation sub-context (or gives every chain/attestation property an explicit
  `@id`) so **unmapped terms are DROPPED, not coerced to `omir:`**; **(c)** attaches PROV to the
  bearing resource via **explicit PROV relations whose subject is the `omir:MemoryRecord` node**
  (the resource keeps its single `omir:` `@type` and *gains* PROV edges) rather than giving it a
  conflicting second `@type` of `prov:Entity`. The disjointness assertion is **narrowed**:
  *"OMIR Entity-as-subject-matter is NOT `prov:Entity`; any OMIR resource appearing as a provenance
  derivation operand lifts to `prov:Entity` for the derivation graph"* (publish that, not a blanket
  `owl:disjointWith` that would make a `wasDerivedFrom Episode` triple ill-formed or the graph
  inconsistent). A normative **omir-role → prov: predicate** table accompanies the chain. **DoD:** a
  worked RDF-output test vector proving two implementers produce identical triples under
  base-only and base+prov, with no spurious coercion artifacts.
- **Redaction mechanics (Phase 4) are concrete.** A redacted resource retains `resourceType`/`id`
  and all reference targets (graph stays valid per CR-5), sets `content` to a defined sentinel
  (`"[redacted]"` + `redacted:true` in the governance block) while its signed `commitment` sibling
  is preserved, and **MUST NOT** be removed while anything references it. Whole-resource deletion
  uses a distinct **tombstone** (a present, conformant resource of the right type). A redaction
  round-trip example ships with the phase.
- **Vendor-neutral by construction.** The worked example uses placeholder agents (`Agent/agent-a`)
  and a placeholder issuer. Any Veld-specific signing convention lives in a vendor extension under
  `veld.dev`, never in the normative core example.

### Overlook — the sequence (lens 1)

| Phase | Themes | Release · OMM | Breaking? | Unlocks |
|---|---|---|---|---|
| 1a / 1b | **A** vocabularies | R2 · OMM-1 | **Union = type-change; R2, RFC-gated. Vocab registry (move 1) ships R1.x-additive; 1b binding-engine off the critical path** | Every non-coding domain can name concepts |
| 2 | **B** Agent + **C** identity | R2 · OMM-0/0 | **New resource + Reference widening; R2, RFC-gated, non-invalidating to R1 bundles. C inert single-party → OMM-0** | Agents to attribute to; cross-store merge (needs 2nd store) |
| 3 | **E** PROV (E1-chain on path; attestation parallel) | R2 · E1-chain OMM-1 / attestation OMM-0 | New optional fields (R2 line) | Verifiable trust across vendors (at P2) |
| 4 | **F** + **D** governance, modality | R1.x where additive¹ | New optional fields | Regulated & multimodal adoption |
| 5 | **G J K** + H, I | R1.x additive / R2+ structural¹ | I's CR-5 carve-out → R2+; G/K/H/F/D **and J's typed-channel + deprecation markers** additive → R1.x¹ | Structured, multilingual, federated memory |

¹ **Genuinely §5.1-additive themes ship as R1.x increments *as they land*** (K, G, H, F, D, the
**non-`Reference` C fields only** — `identifier[]`/`aliases[]`, never `authoredBy`/`owner`/
Agent-`sameAs` — and J's **typed channel + deprecation markers**, since §6.4 makes a deprecation
*marker* a warn-only additive change). The R2 boundary is reserved **strictly** for the actually
non-additive items: the **CodeableConcept union** (type change), the **Agent resource + Reference
widening** (§5.2/§3.1), the eventual **R3 REMOVAL** of J's deprecated string maps, and **I's CR-5
carve-out**. Holding a purely-additive optional field for a major release works against the
cheap-adoption invariant. *Caveat from P1: "R1.x-additive" means "additive to the data model and
deprecation-policy-clean," NOT "forward-readable by an already-shipped same-major closed-schema
validator" — closed schemas reject undeclared new core fields, so intra-major novelty rides
`extension[]` until a minor schema bump admits it.*

### Overlook — dependencies (lens 2)

```
A (vocabularies) ──► everything (domains can finally name their own concepts)
A ──► J (typed values reuse CodeableConcept via the shared TypedValue def; NOT via Extension)
A ──► E1-chain (chain over prior resources; needs no Agent)
B (Agent) ──► E2 (agent-attribution hops attribute memory to Agents) ──► (only) the agent case
B (Reference widening) ──► I (the ExternalReference $def extends, never relaxes, the widened Reference)
C (identifier/sameAs) ──► I (input to how external refs are expressed; not the gate)
I (Federated CR-5 carve-out) ──► attested cross-store E2 (a real Agent or an external ref)
D, F ──► E (close the attestation inclusion allow-list; F's classification enters M)
E (attestation, redaction-aware) ──► F (signed retention/consent; redaction preserves the commitment)
P2 (a second implementer / verifier) ──► OMM-2 anywhere; ──► attestation interop & E maturity at all
```

The critical path is **A → E1-chain**, a pure additive structure reviewable by a second party; the
**attestation subsystem and E2 are a parallel track gated on P2** (cross-verification is their only
test). C runs in parallel after A per the directive, but its *value* (cross-system merge) and I's
mechanism are a matched pair that also need P2. **Maturity floor rule:** a dependent feature's OMM
is `min(grade(its dependencies), grade(its own mechanism), gate(P2 where interop is the test))` —
while `Agent` is OMM-0, E2 and any agent-signing attestation are at most OMM-0; J caps at A's grade;
I and C at single-party OMM-0 until P2. **I's true gate is the CR-5 carve-out RFC + the
`ExternalReference` `$def`, not C.**

### Breakers — adversarial stress-test (3 passes)

**Pass 1 — semantic collisions & compatibility.**
- *PROV `Entity` vs OMIR `Entity`, the base context, and term collisions.* **Mitigation:** distinct
  chain field names (`derivedFrom`/`hopCredibility`, never the globally-bound `from`/`credibility`);
  a separate opt-in PROV context that re-declares `provenance`, suppresses `@vocab` inside the
  subtree, attaches PROV via explicit relations on the `omir:` node (not a second `@type`), and
  publishes the **narrowed** disjointness ("subject-matter Entity ≠ prov:Entity; derivation operands
  DO lift to prov:Entity"). The published context + RDF golden-file vector is what makes the lift
  deterministic.
- *CodeableConcept softens validation.* **Mitigation:** the widened branch is the **existing enum**
  (not free `type:string`), and the seed/object mapping is made **structurally bijective** by
  excluding the field's own seed-system URL from the object branch — so a seed value is expressible
  *only* as the bare string at the Core schema layer, with no Phase-1b/honor-system dependency.
  Phase 1b adds the machine-readable binding construct + net-new validator subsystem for
  closed-vocabulary enforcement.
- *Vocabulary fragmentation — including the field that actually fragments.* **Mitigation:**
  WG-published seed code systems (release-governed) + a public registry (add = additive PR; retire =
  RFC) + a mandatory `text` fallback. **`relationType` is included in A** — the one already-open,
  highest-fragmentation field — so the namespace discipline lands where collisions actually happen.
  Domain vocabularies live under non-omir.io URLs.

**Pass 2 — trust & governance hard parts.**
- *Attestation brittle under re-serialization / across encodings.* **Mitigation:** one canonical
  form `SHA-256(det-CBOR(M))` over a versioned inclusion allow-list, JSON signers build identical M,
  numbers normalized to shortest-round-trip decimal pre-CBOR (floats forbidden), CodeableConcept
  folded bijectively, **dtype-pinned binary forms barred from signed commitments** (and embeddings
  excluded by default), **forbid bincode under attestation (encodings.md edit + validator
  enforcement)**, ship a KAT vector — gated on P2.
- *Retention vs immutable provenance; redaction vs signatures; erasure of a signing Agent.*
  **Mitigation:** signed subset holds **salted per-field commitments, not plaintext** (with an
  **honest unlinkability-not-brute-force claim** and an erasable-nonce option for real resistance);
  redaction deletes only plaintext; **erasure dominates** — Agent identity enters via an erasable
  commitment so a compelled deletion yields `unverifiable-by-erasure`, never `tampered`, and never a
  dangling ref.
- *Key authority across rotation/merge/redaction — and the closed-world invariant.* **Mitigation:**
  **inline the key + key-binding into M** so `verified` is decidable from the bundle bytes alone;
  demote out-of-band DID/HTTPS resolution to Federated; bind to signing-time `at`; never rewrite a
  signed Agent id on merge.
- *Classification across a trust boundary.* **Mitigation:** classification is declarative;
  enforcement is consumer policy; a profile may require the core governance block (and the legacy
  pii-class extension is deprecated with a dual-accept window + value-shape mapping).

**Pass 3 — adoption & process risks.**
- *The whole plan is gated on a second implementer — and nobody is tasked with producing one.*
  **Mitigation: P2 makes the gate a named, dated deliverable, not a deferred "later."** Milestone
  zero is reframed: **a NON-Veld party consumes a published minimal Bundle unchanged and publishes a
  conformance statement** (Conformance §"Declaring conformance"). A is the cheapest *additive spec
  change*, sequenced first to **open the namespace** (move 1, additive, day-one). The independence
  test (P0a) — technical/economic, recusal-bounded — prevents "Veld twice." Co-design C and E *with*
  the first external adopter, named on the RFC record.
- *Backwards compatibility is not a slogan.* **Adding any new core field is a new-release change
  because R1/R2 schemas are closed (`additionalProperties:false`).** Honest guarantee: *"R<n> bundles
  stay valid forever"* + *"R<n+1> readers accept R<n>,"* **not** *"older readers tolerate newer."*
  A published R1 reader rejects any R2 bundle at the envelope (E300) by design. Until a field is
  promoted, data rides `extension[]` under a non-omir.io URL.
- *Scope creep / 80-20 violation — measured by the SPEC SURFACE a second implementer must read, not
  the required-field count.* **Mitigation:** the Adopter floor (below) defines a normatively-labeled
  **Core-R2 mandatory-to-understand tier** (the four+Agent core resources + the CodeableConcept
  text-fallback + ignore-unknown rules) and an **Optional-capability tier** (B-semantics, D, E, F, G,
  H, I, J, K) a Core-R2 consumer **MAY ignore wholesale without reading their specs**. WG effort is
  tied to the gate: **no phase past Phase 2 begins schema work until a candidate second implementer
  has cleared the Phase-1a floor (P2).** "Adopt all" stays the destination without spending the
  cheap-adoption budget before the gate is met. **`Extension.valueJson` stays arbitrary JSON.** OMM
  is graded **per resource TYPE**; new volatile *fields* on a type do not inherit or drag the type's
  grade — but the field-level signal must be machine-readable, not buried in prose: a new field on a
  mature type either **rides `extension[]` until it earns promotion** or carries an
  **`x-omir-maturity` annotation the validator surfaces as an INFO finding** when a bundle uses a
  below-type-grade field (prose `description` is governed as editorial §5.1 and would let stability
  claims escape the OMM ballot gate — so it is not the vehicle).
- *Velocity vs honesty.* **Mitigation:** the OMM rule — **no OMM-2 without ≥2 independent
  implementations** — is the brake, real only once P0 reconciles the three conflicting definitions
  *and re-grades the four overclaimed R1 types*. A **falling trigger** keeps the gate visible: any
  feature holding OMM-1 across a full release cycle (or fixed calendar window) **with no recorded
  independent implementation MUST be flagged** "OMM-1 (single-party; no independent implementation as
  of <date>)" and becomes a candidate for §4.2 TSC review — so a standard stuck single-party forever
  is **not** indistinguishable in the grade table from a healthy one.

### Adopter conformance floor

So a second implementer can scope the work, the **Core-R2** floor is explicit, split into a
**mandatory-to-understand** tier and an **optional-capability** tier.

**Mandatory to understand (the whole of Core-R2 reading):** the five core resources (MemoryRecord,
Entity, Relationship, Episode, Agent), the CodeableConcept union + `text` fallback, and the
ignore-unknown rules. A Core-R2 consumer:
- **MUST** parse the CodeableConcept union and read `text`; **MAY** ignore `system`/`code` it does
  not know.
- **MUST** ignore the *semantics* of unknown Agents, chain hops, governance blocks, embeddings,
  typed attributes, and i18n variants without rejecting (extending "ignore unknown extensions" to
  new optional core blocks).
- **MUST** satisfy reference integrity as a **document property** for Bundles it **AUTHORS**: a
  Bundle a Core-R2 implementation authors **MUST have a closed chain** (every `chain[].agent`/
  `chain[].derivedFrom` resolves in-Bundle), exactly as CR-5 is unconditional. **When RELAYING a
  Bundle it did not author, it MUST pass the chain through unmodified (lossless pass-through) and
  MUST NOT introduce a new dangling ref** — a trust-agnostic relay is never forced to mint
  placeholder Agents into a chain it cannot interpret, and the Phase-3 "producer MUST emit a closed
  chain" wording means *author*, not *relay*.

**Optional capability (may be ignored wholesale, specs unread):** attestation verification,
federation references, Agent/B semantics beyond ignore-and-preserve, D modality, F governance
enforcement, G/H/J/K. A Core-R2 implementation is **NOT** required to verify attestations, resolve
federation references, or implement any profile it does not claim.

A Core-R2 **producer** MUST emit the backward-compatible (bare-string) form where it has no coded
value, and MUST place implementation-specific data in `extension[]` under a non-omir.io URL.

### Definition of done (per phase)

A phase reaches **its stated target maturity** when, for each theme it lands: the **schema** and the
**version-aware reference validator** support it — including that **every new `ResourceType/id`
reference field (and every closed-world bare-id field, new and pre-existing) is in the
registry-driven reference walk and exercised by a dangling-reference invalid example in
`examples/invalid/`**. The **non-regression clause is split by the phase that introduces the field**,
because a field cannot have a fixture before it exists:
- **Phase 2 DoD (the walker-refactor guard):** a dangling-`parentId` fixture **AND** a dangling
  `Relationship.sourceEpisode` fixture MUST be added and MUST still be rejected, asserted in
  `validator/tests/conformance.rs` **before the registry-walker refactor merges** (parentId is the
  lib.rs:219-230 special case the refactor risks dropping; sourceEpisode was the silently-omitted
  pre-existing field).
- **Phase 3 DoD:** dangling `chain[].derivedFrom` and `chain[].agent` fixtures, **plus** a
  wrong-shape `chain[].agent` (e.g. resolving to `Episode/x`) fixture proving the broadened
  set-membership E201 still rejects a non-Agent in the Agent slot.

For **A**: a **forward-compat fixture PAIR**, each with a single unambiguous expected code —
**(i)** an **R2-declared** bundle with object-form CodeableConcept that the **R1 validator rejects
with E300 at the envelope** (proves "R1 readers reject R2 by design"); and **(ii)** an
**R1-declared** bundle that nonetheless contains an object-form CodeableConcept, which the **R1
schema set rejects with E120 at per-entry dispatch** (proves the legacy enum stays hard-validated).
Plus the `@context` Turtle golden-file (the `source`-split audit) and the intention-filtering
object-form fixture. For **1b**: profile-constraint checking enforces code-system membership with
passing/failing examples. For **E**: the cross-verifier known-answer attestation vector, the
bincode-under-attestation failing example, the default-vs-omitted and `9`-vs-`9.0` digest-equality
KATs, and the RDF golden-file — the *interop* DoD items gated on P2. At least one **profile**
exercises each theme; **docs + examples** exist; an **OMM grade** is assigned honestly under the
reconciled, re-graded P0 ladder.

**The independent-implementation requirement applies to claiming OMM-2+, not to completing a phase
— but P2 binds at least one phase to a real second party.** OMM-0/1 = the reference implementation
alone; OMM-2 = ≥2 independent (per P0a). Phases 1–5 may ship at OMM-0/1 on the reference
implementation alone, so the standard ships **before** a second implementer materializes — directly
serving "cheap early adoption matters more than feature count." **But Phase 1a MUST NOT be declared
done until a candidate independent implementer (named on the RFC/ballot record) has either (a)
consumed an R2 CodeableConcept-union bundle and round-tripped it, or (b) recorded a public statement
of intent** — so the one existential invariant is a gate, not an owner-less "later." **Process
preconditions (P-1, P0, P0a, P1) must land before the phases they gate** — no phase may claim OMM-2
before the OMM reconciliation, no R2 traffic is testable before the validator is version-aware, and
no ballot is callable before the founding TSC is seated.

## Process

None of this is unilateral. Every change here is a **candidate**, to be proposed as an RFC,
debated by the Working Group, and balloted (see [`CONTRIBUTING.md`](https://github.com/OMIR-Working-Group/OMIR/blob/main/CONTRIBUTING.md)
and [`GOVERNANCE.md`](https://github.com/OMIR-Working-Group/OMIR/blob/main/GOVERNANCE.md)).
New resources and fields enter at **OMM-0/1** and earn maturity through *independent*
implementation — the same honesty rule the rest of the spec lives by. The fastest way to
move any of these forward is the project's stated existential need: **a second implementer**
whose requirements turn one of these themes from a hypothesis into a ballot.
