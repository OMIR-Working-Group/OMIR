# Memory Semantics

The resource pages define *what fields a record carries*. This page defines *what those
fields mean over time* — the cognitive model OMIR encodes. It is the answer to a fair
question about the field tables: "why is confidence two numbers and a third? what is a
half-life doing in a data format? what makes an edge's strength go up?"

OMIR's guiding rule, restated from [Design Principles §6](./principles.md#6-forgetting-is-first-class):

> **OMIR standardizes the *state* a memory algorithm reads and writes — not the algorithm.**

Two faithful implementations may decay, consolidate, and re-rank memory with completely
different code. What they must agree on is the *meaning* of the state in the file. The
mechanisms below are drawn from the reference implementation (Veld); the spec records the
state, and an implementation is free to reproduce the behavior however it likes.

## 1. Confidence as calibrated belief

`MemoryRecord.confidence` is a `Confidence` object — `{ alpha, beta, calibrated }` — not a
bare float, because an agent needs to distinguish *"90% sure, having checked many times"*
from *"90% sure, having seen this once."* A single number cannot.

OMIR models belief as a **Beta(α, β) posterior**:

- `alpha` accumulates **confirming** evidence (the memory proved helpful/correct), plus a prior.
- `beta` accumulates **disconfirming** evidence (the memory proved misleading/wrong), plus a prior.
- `calibrated` is the point estimate a consumer should *act* on. The natural estimate is
  the posterior mean, `α / (α + β)`, but a faithful producer **damps it toward 0.5 when
  evidence is thin** — at one or two observations the prior should dominate, so an
  unproven memory does not masquerade as a confident one. As `α + β` grows, `calibrated`
  approaches the raw mean. R1 does **not** mandate the exact shrinkage, so `calibrated` is a
  *producer-asserted* point estimate: two faithful producers MAY emit a different
  `calibrated` for the same `(α, β)`. The `α`/`β` pair is the portable, reproducible
  quantity; a consumer that needs a reproducible or cross-producer-comparable estimate
  **SHOULD** recompute it from `α`/`β` under its own rule rather than rely on `calibrated`
  being identical across producers (see
  [Theory & Scope](./theory.md#stored-scalars-are-producer-relative)).

A consumer that only reads `calibrated` behaves correctly *within a single producer's
records*; the `α`/`β` pair is there for consumers that want to keep updating belief as new
feedback arrives. This is why confidence is a distribution rather than a bare float — it
carries its own evidence count, so belief can be *revised* rather than merely averaged.
**One caveat on merging:** adding two stores' `α`/`β` yields a coherent posterior **only when
the two evidence streams are independent**. When both stores derived their belief from the
same upstream source — common when memories are exported and re-imported — summing the counts
double-counts the shared evidence and overstates confidence. A consumer that merges
confidence across stores **SHOULD** account for shared provenance (e.g. via
`provenance.externalId`) rather than blindly add counts.

## 2. Forgetting as a curve

`MemoryRecord.decay` (`{ halfLifeHours, lastAccess, accessCount, anchored }`) records a
memory's **forgetting state** so that "better forgetting" survives export. The intent:

- A record's retrievability falls over time on a **half-life** — recent, frequently
  accessed memories stay sharp; stale ones fade. `lastAccess` and `accessCount` are the
  inputs a decay function reads; each access pushes retrievability back up.
- Decay is naturally **multi-time-scale**: fast in the first hours/days (filtering noise),
  then much slower for what survives — empirically closer to a power law than a single
  exponential. OMIR does *not* mandate the curve; it records the parameters
  (`halfLifeHours`) and the access signal a curve consumes.
- **`anchored: true`** marks a memory that resists decay — a pinned fact, a user-stated
  preference, a safety constraint. Anchored memories have a floor below which
  retrievability does not fall.

The spec stores forgetting *state*, not a forgetting *algorithm*, precisely so a robot on a
power-cycle budget and a cloud assistant can both reconstruct sensible decay from the same
fields.

## 3. Tiering and consolidation

`MemoryRecord.tier` (`working → session → longterm → archive`) records where a memory sits
in a consolidation hierarchy — the at-rest analogue of working vs. long-term memory:

- **working** — active, in-focus, short-lived; the smallest, hottest set.
- **session** — bounded to a task/conversation; indexed for fast recall.
- **longterm** — consolidated knowledge, retrieved by semantic cue.
- **archive** — cold, near-permanent, batch-retrieved.

Promotion is driven by **age × importance × access**: a memory accessed repeatedly, or
marked important, or linked to long-term knowledge, migrates inward; an untouched
low-importance memory drifts outward and eventually compresses. OMIR records the *tier* a
record currently occupies; the promotion policy is the implementation's.

## 4. Hebbian relationship strength

`Relationship.strength` is a **synaptic weight**, not a static label. The cognitive model is
Hebbian — *cells that fire together wire together*:

- When two entities are **co-activated** (retrieved or mentioned together), the edge between
  them **strengthens**.
- An edge that is not used **decays** toward zero, the same "better forgetting" applied to
  structure rather than content.
- `validAt` records when the relationship was last observed to hold; `invalidatedAt` retires
  an edge that has been contradicted **without deleting it**, so the graph keeps its history
  rather than silently rewriting it.

Heavier machinery a faithful engine may run on top of this — long-term potentiation,
asymmetric forward/backward strengths, consolidation tiers on the edge itself — is
*implementation state* and rides in `extension[]`, not the core. The core carries the one
number every graph engine agrees on: current strength.

## 5. Salience

`Entity.salience` is how strongly an entity pulls on retrieval — its gravitational mass in
the memory space. A high-salience entity is more likely to be surfaced and to drag related
memories up with it via spreading activation. Salience rises with **frequency**
(`mentionCount`), **recency** (`lastSeenAt`), whether the entity is a **proper noun**
(`properNoun`), and explicit user importance. As with decay, OMIR stores the salience value
and its inputs; how an engine computes and uses it is its own business.

## 6. The temporal model

OMIR is **bi-temporal in spirit**: it separates *when something happened* from *when it was
recorded*.

- `eventTime` — when the described event actually occurred. It **may precede** `createdAt`
  (you can record a memory of last week today).
- `createdAt` — encoding time: when the record was written.
- `validUntil` (on `MemoryRecord`) and `invalidatedAt` (on `Relationship`) express
  **temporal invalidation** — a fact superseded after an instant, or an edge that stopped
  holding — so consumers can filter stale knowledge rather than trust it forever.

Entry order is **not** significant; everything resolves by `ResourceType/id`. A consumer must
never infer meaning from position ([Conformance](./conformance.md)). A richer, fully
interval-based bitemporal model is a candidate generalization — see
[Toward a Global Standard §H](./global-standard.md#h-a-complete-temporal-model).

## 7. Provenance and source credibility

`MemoryRecord.provenance` (`{ source, sourceType, credibility, externalId }`) answers *where
a memory came from and how much to trust the origin*. `credibility` (a `[0,1]`
`UnitInterval`) lets a consumer weight a memory by the trustworthiness of its source — a
user statement, a verified document, and an inferred guess are not equally reliable, and a
retrieval ranker should be able to say so. `externalId` (`system:id`, e.g. `github:pr-123`,
`linear:SHO-39`) keeps a memory traceable back to the artifact that produced it. A
multi-hop, signable provenance chain is a candidate generalization — see
[Toward a Global Standard §E](./global-standard.md#e-provenance-and-trust-as-a-chain).

## 8. Prospective memory

Most memory is about the past. **Prospective memory** is about the future: a remembered
*intention* to do something later. OMIR models it with `experienceType: "intention"` on an
otherwise ordinary `MemoryRecord` — future-directed records live in the same store as
retrospective ones, so the same decay, confidence, and provenance machinery applies. A
conforming consumer **SHOULD** keep intentions out of ordinary retrospective recall and
surface them when their trigger condition is met (a time, a context); the
`omir-personal-assistant` profile makes this explicit ([Profiles](./profiles.md#omir-personal-assistant)).

## What OMIR deliberately does *not* specify

The following are *implementation* concerns. They are real and important, but they are not
part of the at-rest contract, and a conformant document says nothing about them:

- **Retrieval scoring.** How memories are ranked at query time (similarity, BM25, graph
  spreading, cross-encoders, RRF fusion, multi-signal blends) is engine-specific. Score
  vectors ride in `extension[]`; they are never core. See the worked example in
  [Extensions](./extensions.md#worked-example-velds-20-signal-retrieval-scores).
- **Embeddings.** Which model, which dimensionality, which vector — all implementation. R1
  carries embeddings only as extensions; a neutral embedding representation is a candidate
  generalization ([Toward a Global Standard §D](./global-standard.md#d-beyond-text-modality-and-embeddings)).
- **Consolidation schedules, replay, "sleep" phases.** When and how memory is reorganized
  is an algorithm, not a state.
- **Storage and indexing.** Vector indexes, key-value engines, graph databases — none of it
  is OMIR's concern. OMIR is the bytes at rest, not the engine.

This boundary is the whole design: standardize the **state** so any engine can read another
engine's memory, and leave the **behavior** free so engines still have something to compete
on.
