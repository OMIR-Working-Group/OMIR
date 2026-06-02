# Theory & Scope

> **Non-normative.** This page explains the theory of memory OMIR R1 encodes and states the
> boundaries of that theory. It adds **no** conformance requirements beyond those in
> [Conformance](./conformance.md); the RFC-2119 keywords below restate consumer guidance
> already implied by the resource semantics. It is the conceptual companion to
> [Memory Semantics](./semantics.md), which defines how individual fields behave over time.

## The theory OMIR encodes

OMIR R1 standardizes the **state** a memory algorithm reads and writes, not the algorithm
([Design Principles §6](./principles.md#6-forgetting-is-first-class)). The state it makes
first-class — calibrated belief, half-life decay with anchoring, consolidation tiers,
Hebbian edge strength, entity salience, bi-temporal timestamps, prospective intentions, and
source credibility — places OMIR in a specific, well-established tradition: the **rational
analysis of memory**, in which a memory's strength tracks the statistics of the environment
(recency and frequency predict future need), realized in cognitive architectures such as
ACT-R and complemented by the hippocampal/neocortical picture of consolidation.

Naming the lineage matters because "state, not algorithm" does **not** make the *choice* of
state neutral. The fields OMIR blesses are an opinionated, defensible theory of what memory
is. This page states the boundaries of that theory so they read as deliberate design
positions, not as omissions.

## Scope: declarative memory

OMIR R1 models **declarative memory** — the episodic and semantic memory of facts,
experiences, and their relationships. In the common taxonomy of agent memory (a working
store plus long-term *episodic*, *semantic*, and *procedural* memory), R1 covers the
episodic and semantic long-term store, with `tier` standing in for a working/activation
notion.

The following are **out of scope** in R1 and have no first-class representation:

- **Procedural / skill memory** — learned tool-use policies, workflows, and macros.
- **Parametric memory** — knowledge held in model weights, adapters (e.g. LoRA), or
  fine-tunes.
- **Activation / KV-cache memory** — transient in-context state below the record level.
- **Priming and other implicit (non-declarative) memory.**

Such state **MAY** be carried opaquely (inside `content`, or under `extension[]`), but it is
not interpretable by a generic consumer and is **not** what R1 standardizes. A later release
**MAY** introduce first-class resources for these systems; per
[Design Principles §4](./principles.md#4-honest-maturity-via-omm) they would start low on the
OMM and earn their level through independent implementation. Until then, "OMIR memory" means
*declarative* memory — stated here so adopters self-select rather than discover the boundary
after building an adapter.

## State, not dynamics — and the snapshot it implies

Because OMIR records state and not the algorithm that evolves it, every time-dependent value
is a **snapshot as of its timestamp**. A `decay` block, an `Entity.salience`, a
`Relationship.strength`, and the retrievability they feed are all functions of time, frozen
at the moment of export. R1 defines **no** procedure for aging them forward.

A consumer **SHOULD** treat these values as valid *as of* their associated timestamp
(`decay.lastAccess`, `Entity.lastSeenAt`, `Relationship.validAt`, `meta.lastUpdated`) and
**MAY** recompute them under its own decay / salience / strength model on import. A consumer
**MUST NOT** assume an exported retrievability, salience, or strength is current at read
time. This is the price of encoding-neutral, algorithm-free portability: the record survives
the hand-off; the dynamics are reconstructed by the importer.

## Stored scalars are producer-relative

The normalized scores OMIR carries — `importance`, `Entity.salience`,
`Relationship.strength`, `provenance.credibility`, and `confidence.calibrated` — are
**producer-asserted and producer-normalized**. They are comparable *within* one producer's
output, where they share a normalization regime, and are **not** guaranteed comparable
*across* producers.

R1 defines no cross-producer scale for these fields. Consequently:

- A consumer **MUST NOT** assume two producers' scalars share a scale: one store's
  `importance: 0.8` need not mean what another store's `0.8` means.
- A consumer that ranks or merges records **from more than one producer** **SHOULD**
  renormalize per producer (keyed by `meta.source`) rather than compare raw values.
- For belief specifically, prefer the evidence-bearing `confidence.alpha` / `beta` over the
  derived `calibrated` when comparing or merging across producers
  ([Memory Semantics §1](./semantics.md#1-confidence-as-calibrated-belief)).

This reflects a fact about memory the literature is explicit on: salience and relevance are
**cue-dependent and emergent at retrieval**, not context-free stored properties. OMIR must
freeze them into stored scalars to serialize them at all; this section bounds what that
freezing does and does not guarantee. A future release **MAY** add an optional
`normalizationRef` so cross-producer comparability becomes *detectable* rather than assumed
(see [Toward a Global Standard](./global-standard.md)).

## Records, not reconstructions

Human recall is **reconstructive**: a trace is rebuilt, and often altered, each time it is
retrieved. OMIR deliberately does **not** model this. A `MemoryRecord` is a stable, auditable
record with an explicit `version` counter and temporal *invalidation* (`validUntil`,
`Relationship.invalidatedAt`), not a trace that reconsolidates on access. Accessing a record
updates its retrievability (`decay.lastAccess`, `decay.accessCount`); it does not rewrite its
`content`.

This is a deliberate trade: OMIR exchanges reconstructive fidelity for **auditability,
diffability, and non-confabulation** — properties a portable, archivable,
hand-it-between-vendors format needs and a reconstructive store cannot offer. It also means
OMIR's "forgetting" is decay of *retrievability*, never deletion of data; governance-driven
deletion and retention are a separate concern, deferred to a future governance vocabulary
(see [Toward a Global Standard](./global-standard.md)).

## These are bounds, not gaps

Each boundary above is a stated position, not an oversight. Several have candidate
generalizations already named in [Toward a Global Standard](./global-standard.md) — open
vocabularies, identity and multi-agent memory, a richer temporal model, modality and neutral
embeddings, and a governance / retention vocabulary. They enter, if at all, through the RFC +
ballot process at low OMM. R1's contract is the narrower, honest one: a portable
serialization of *declarative* memory *state*.
