# Efficiency & Information-Bearing Codes

> **Non-normative.** This page is a design discussion, not part of the R1 conformance
> surface. Nothing here changes what makes a Bundle valid ([Conformance](./conformance.md)).
> Every change below would enter through the RFC + ballot process and start low on the
> [OMM](./principles.md#4-honest-maturity-via-omm) — **OMM-0/1** — and earn its level through
> independent implementation. It is the **efficiency-first** companion to
> [Toward a Global Standard](./global-standard.md) (the interchange-first schema roadmap) and
> the forward-looking counterpart to [`memory_theory.md`](https://github.com/omir-wg/omir-standard/blob/main/memory_theory.md) (the
> backward-looking divergence map). The live R1 schemas are **not** edited here — R1 resources
> set `additionalProperties:false` (CR-6), so every proposal is an R2-line / R1.x candidate,
> stated as a delta, never applied to the frozen R1 set.

Where [Toward a Global Standard](./global-standard.md) asks *"what blocks interchange across
domains?"*, this page asks a different question: *"what state would let an engine store less,
search cheaper, and infer more — at the same fidelity?"* The seven proposals below come from
the efficient-/predictive-coding, fuzzy-trace, hippocampal-indexing, event-segmentation,
temporal-context, and chunking lineages. Each is scored on three axes the interchange roadmap
does not track: **⚡ watts** (store/search less), **🎯 inference** (signal-to-noise), and
**capability** (what becomes expressible).

## The one-paragraph thesis

These are **not** seven scattered fields. **Four of the seven (EP-2, EP-3, EP-5a, EP-F) are
variants of a single object** — the algorithm-neutral `Embedding` representation that
[global-standard §D](./global-standard.md#d-beyond-text-modality-and-embeddings) already
proposes. The highest-leverage move is therefore a reframing the divergence map already named:
**extend Theme D from "neutral embeddings" to *efficiency-bearing* codes** — an `Embedding`
that can carry a Matryoshka prefix (rank coarse-to-fine), a sparse code (CPU inverted-index
search), a reserved drift space (cheap temporal cues), and a VSA structural code (compositional
ops). The remaining three are: two **information-theoretic scalars OMIR has no field for**
(EP-1 surprisal, EP-4 replay/interference), and one **graph fix that makes spreading
activation portable** (EP-6c, the D5 edge-normalization remediation). Two proposals add genuinely
new structure: an `Episode` event-boundary (EP-4) and a `Chunk` consolidation-product resource
(EP-6a). One adds a producer-level metamemory primitive (EP-5b, the familiarity sketch).

## Proposal index

| EP | Prescription (from the principle) | Concrete delta | Additivity | Vehicle · OMM | Theme | Divergence |
|---|---|---|---|---|---|---|
| **EP-1** | surprisal/novelty scalar + model-redundancy flag | new `InformationContent` def + `MemoryRecord.informationContent` | §5.1-additive | RFC-gated · **R1.x** · OMM-0 | *new* | — (new lever) |
| **EP-2** | Matryoshka gist + offloadable verbatim | extend Theme-D `Embedding`: `matryoshka`/`nestedDims`/`role` + verbatim `MediaReference` | additive to the D def | rides **D · R2** · OMM-0 | **D** | D2 (partial) |
| **EP-3** | sparse index layer (indices+values) | `Embedding.sparse` + Theme-I external-content pointer | additive to the D def | rides **D · R2** (+ I · R2+) · OMM-0 | **D + I** | — |
| **EP-4** | boundary metadata + replay priority + interference | `Episode.boundaryStrength/boundaryReason`; `MemoryRecord.replayPriority`; new `Interference` def + field | §5.1-additive | RFC-gated · **R1.x** · OMM-0 | *new* | **D10** (closes), D8/D4 |
| **EP-5** | temporalContext drift vector + familiarity sketch | reserved Embedding space `omir:temporal-context` (D); new `FamiliaritySketch` def + `Bundle.familiaritySketch` | sketch §5.1-additive; tc rides D | sketch **R1.x** · OMM-0; tc rides **D · R2** | **D** + *new* | — |
| **EP-6** | chunk(`composedOf`) + schema typing + edge-norm fix | **new `Chunk` resource** (Templates = reusable Chunks); `MemoryRecord.schemaType` (CodeableConcept); `Relationship.reverseStrength/normalizedStrength/normalization` + `Check::GraphNormalization` (E250/E251) | Chunk = §3.1/§5.2 **breaking**; edge-norm §5.1-additive; schemaType rides A | Chunk **R2 · OMM-0** (§4 RFC); edge-norm **R1.x** · OMM-0; schemaType rides **A · R2** | **A** + graph + *new* | **D5** (closes), D8 |
| **EP-F** | VSA / HRR structural code (frontier) | Embedding `space:"vsa"` + structure tags; **extension-first** | extension (no RFC) → promotable | extension now; **D · R2+** · OMM-0 later | **D** + `.omirb` profile | — |

Routing convention (from [GOVERNANCE §3.1/§5.1/§5.2](https://github.com/omir-wg/omir-standard/blob/main/GOVERNANCE.md)): a **new optional
field** is §5.1-additive (no existing bundle becomes invalid → ships as an **R1.x** increment)
but still touches the normative surface, so it is **RFC-gated** (the RFC authorizes the schema
edit; R1.x is the release lane). A change that **widens the shared `Reference` pattern or the
`Bundle.entry` oneOf** (a new resource type) is §5.2-breaking → **R2**, full RFC per
[CONTRIBUTING §4](https://github.com/omir-wg/omir-standard/blob/main/CONTRIBUTING.md). Adding a new, unreferenced `$defs` member to
`common.schema.json` is itself purely additive; it is the field that *references* it that
carries the additivity class. New fields ride at **OMM-0** under the per-field `x-omir-maturity`
signal ([global-standard, Pass-3](./global-standard.md#breakers--adversarial-stress-test-3-passes)),
never inheriting their host type's grade.

---

## EP-1 — Information content (don't store what the model can regenerate)

**Principle 1 (efficient coding · predictive coding).** *"A surprisal/novelty scalar at
encoding (information content vs. the producer's model) and a model-redundancy flag
(reconstructable from the base model?). This is not importance (value) or confidence (belief) —
it's information content, which OMIR has no field for."*

The orthogonality is the whole point: `importance` is **value**, `confidence` is **belief**,
this is **information** (−log p against a generative prior). A record the base model already
knows is near-zero information regardless of how important or believed it is — and is the single
biggest watt lever, because most "memories" are model-knowable and need not be persisted or
searched at all (cf. Titans' ‖∇loss/∇input‖ write-gate, EM-LLM's Bayesian surprise).

**New `common.schema.json#/$defs` member (purely additive):**

```json
"InformationContent": {
  "type": "object",
  "description": "Information content of a record against a generative prior — what is NOVEL, distinct from what is valued (importance) or believed (confidence). Operationalizes efficient/predictive coding: store the surprising, regenerate the predictable.",
  "properties": {
    "novelty": {
      "$ref": "#/$defs/UnitInterval",
      "description": "Producer-relative normalized novelty in [0,1]: how surprising this record was against the producer's model at encoding. Comparable WITHIN one producer only (Theory & Scope: stored scalars are producer-relative)."
    },
    "surprisalBits": {
      "type": "number",
      "minimum": 0,
      "description": "Raw Shannon surprisal -log2 p(x) in bits against the model named in 'model'. Unbounded and model-relative; the reproducible quantity when 'model' is shared."
    },
    "model": {
      "type": "string",
      "description": "Identifier of the generative model whose p(x) defines 'novelty'/'surprisalBits', e.g. 'minilm-l6-v2' or a base-LLM id."
    },
    "reconstructable": {
      "type": "boolean",
      "description": "True if this content is regenerable from the model named in 'reconstructableBy' and is therefore a candidate to DROP and regenerate rather than store/search."
    },
    "reconstructableBy": {
      "type": "string",
      "description": "Identifier of the model that can regenerate this content when 'reconstructable' is true."
    }
  },
  "additionalProperties": false
}
```

**`MemoryRecord` additive field:**
`"informationContent": { "$ref": "common.schema.json#/$defs/InformationContent" }`.

**Classification.** New optional field on MemoryRecord; no enum/`required`/`Reference` change →
**§5.1-additive**, ships as **R1.x**, RFC-gated for the normative meaning. **OMM-0.** `novelty`
is a `UnitInterval` (CR-7 conformant) and a producer-relative snapshot → on landing it joins the
[Theory & Scope](./theory.md#stored-scalars-are-producer-relative) producer-relative list;
`surprisalBits` is the model-relative reproducible form.

**Hits.** ⚡⚡⚡ watts (persist/search only the genuinely novel) · 🎯 inference (signal-to-noise) ·
capability (compression). **Open questions:** does `reconstructable:true` license a consumer to
omit `content` entirely (ties EP-2 verbatim-eviction)? bits vs nats — pin one (`surprisalBits`,
log2, here).

---

## EP-2 — Matryoshka gist + offloadable verbatim (store the gist, fetch the words)

**Principle 2 (fuzzy-trace · rate-distortion).** *"A dual-trace model: a compact, durable,
anchorable gist code (cheap to rank, slow decay) + an optional, fast-decaying, offloadable
verbatim payload (via a MediaReference). Crucially, mandate the gist be prefix-truncatable
(Matryoshka-style) so coarse-to-fine works across producers."*

This is the **efficiency-bearing-codes** extension of Theme D, stated literally. Matryoshka
representation learning ([2205.13147](https://arxiv.org/abs/2205.13147)) is the exact engineering
analog: rank millions on a truncated prefix, re-rank survivors on more dims. The dual trace maps
the gist to a durable anchorable `Embedding` and the verbatim surface to an offloaded
`MediaReference` (Theme D) with its own faster decay.

**Extends the Theme-D `Embedding` def with three efficiency fields:**

```json
"matryoshka": {
  "type": "boolean",
  "description": "True if 'vector' is a nested (Matryoshka) code: any prefix whose length is in 'nestedDims' is itself a valid, rankable embedding. Enables coarse-to-fine shortlisting on a truncated prefix without re-embedding."
},
"nestedDims": {
  "type": "array",
  "items": { "type": "integer", "minimum": 1 },
  "description": "Ascending valid prefix lengths, e.g. [64,128,256,512,768]. A consumer MAY rank on any listed prefix and re-rank on a longer one. Each prefix is comparable only within the same 'space'."
},
"role": {
  "enum": ["gist", "verbatim"],
  "description": "Dual-trace role. 'gist' is the durable, compact, anchorable code ranked cheaply (pair with decay.anchored + long halfLifeHours); 'verbatim' is the fast-decaying surface trace, typically offloaded via 'ref' (MediaReference) and fetched only for top-k."
}
```

**Decay split.** The gist reuses the existing `Decay` block (`anchored:true`, long
`halfLifeHours`); the verbatim trace carries a short `halfLifeHours` and is the first thing
dropped under memory pressure — graceful degradation that keeps the rankable gist.

**Classification.** Additive to the (not-yet-landed) Theme-D `Embedding` def → **rides Theme D ·
R2 · OMM-0**. No `Reference`-pattern impact (`MediaReference` is `{uri,contentType,hash}`, not a
typed `Reference`). **Closes part of D2** (storage/retrieval-strength duality): the durable gist
is the storage-strength-bearing trace, the evictable verbatim is retrieval-strength-bearing.

**Hits.** ⚡⚡ watts (coarse-to-fine + verbatim eviction) · 🎯 inference (gist generalizes) ·
capability (summary recall). **Open question:** make `nestedDims` ascending + power-of-two by
convention so two producers' prefixes align for cross-store shortlisting.

---

## EP-3 — Sparse index layer (index, don't scan)

**Principle 3 (hippocampal indexing · SDM · sparse codes).** *"Separate a cheap index layer
(sparse keys/pointers) from content (offloadable); standardize a sparse-code representation
(indices+values). The index may point at content living elsewhere (ties to federation, §I)."*

A sparse code turns ANN-on-GPU into an exact inverted-index lookup on CPU (≈ an order of
magnitude cheaper), and modern-Hopfield / SDM gives one-step associative completion. The
"content lives elsewhere" half is exactly [global-standard §I](./global-standard.md#i-federation-and-cross-bundle-references)'s
federation: the index entry resolves to remote content via the `ExternalReference` `$def` (never
the closed-world bare `ResourceType/id`).

**Extends the Theme-D `Embedding` def with a sparse variant (alternative to dense `vector`):**

```json
"sparse": {
  "type": "object",
  "description": "Sparse code: parallel 'indices'/'values' over a 'dims'-wide space. Enables CPU inverted-index search and one-step associative completion (Sparse Distributed Memory / modern Hopfield). Mutually exclusive with 'vector' for a given Embedding.",
  "properties": {
    "indices": { "type": "array", "items": { "type": "integer", "minimum": 0 }, "description": "Active dimension indices, strictly ascending." },
    "values":  { "type": "array", "items": { "type": "number" }, "description": "Weights parallel to 'indices'. Equal length to 'indices'." }
  },
  "required": ["indices", "values"],
  "additionalProperties": false
}
```

**Classification.** Additive to the Theme-D `Embedding` def → **rides D · R2 · OMM-0**; the
pointer-to-remote-content rides **Theme I · R2+** (its true gate is I's CR-5 carve-out +
`ExternalReference` `$def`, not this field). The index/content split composes with EP-2's
gist/verbatim and EP-1's `reconstructable` drop.

**Hits.** ⚡⚡⚡ watts (CPU sparse search + lazy content load) · 🎯 inference (DG-style
pattern-separation, anti-interference) · capability (recall from fragments). **Open question:**
`indices` length cap / a `density` hint so a consumer can pick inverted-index vs dense path.

---

## EP-4 — Boundary, replay priority, interference (encode by surprise, prune by interference)

**Principle 4 (event segmentation · prioritized replay · rational forgetting).** *"Event-boundary
metadata on episodes (location + boundary strength + why); a replay/consolidation priority
(surprise × value × recency); an eviction/interference signal (need-probability or local
embedding density)."*

Three deltas, the third of which **closes divergence D10** — the doc's explicit reframe of D10
from *"missing fidelity"* (a bounded non-goal) to *"missing the rational-eviction efficiency
mechanism"* (a load-bearing watt lever: a smaller hot index makes every query cheaper).

**(a) `Episode` additive fields** — the EM-LLM / Event-Segmentation boundary:

```json
"boundaryStrength": {
  "$ref": "common.schema.json#/$defs/UnitInterval",
  "description": "Prediction-error / Bayesian-surprise magnitude at this episode's boundary. Where memory is structured; a segmentation cue for downstream consolidation."
},
"boundaryReason": {
  "type": "string",
  "description": "Open vocabulary: why the cut was made (e.g. 'topic_shift', 'temporal_gap', 'actor_change'). Promotable to a CodeableConcept under Theme A."
}
```

**(b) `MemoryRecord.replayPriority`** — the prioritized-replay (Schaul 2015) weight:

```json
"replayPriority": {
  "$ref": "common.schema.json#/$defs/UnitInterval",
  "description": "Producer-relative consolidation/replay priority (surprise x value x recency snapshot). Prioritizes amortized OFFLINE re-embedding/consolidation/decay to idle time. Snapshot as of meta.lastUpdated; producer-relative (Theory & Scope)."
}
```

**(c) New `Interference` def + `MemoryRecord.interference` field** — the D10 remediation:

```json
"Interference": {
  "type": "object",
  "description": "Rational-eviction / interference signal: why a record competes for retrieval and how prunable it is. Closes divergence D10 — forgetting as interference (retroactive/proactive competition among similar traces), not pure time-decay.",
  "properties": {
    "needProbability": {
      "$ref": "#/$defs/UnitInterval",
      "description": "Anderson need-probability: estimated P(this record is needed soon). The rational eviction key — evict lowest need, NOT oldest (LRU). Distinct from decay (time) and importance (value)."
    },
    "localDensity": {
      "type": "number",
      "minimum": 0,
      "description": "Nearest-neighbour crowding in embedding space. High density = high interference from similar traces (the dominant real forgetting mechanism in similarity-based stores)."
    },
    "competesWith": {
      "type": "array",
      "items": { "$ref": "#/$defs/Reference" },
      "description": "MemoryRecord references this record competes with — similar traces that degrade each other's retrievability. Reuses the existing Reference pattern (MemoryRecord already in it); no widening."
    }
  },
  "additionalProperties": false
}
```

**Classification.** All §5.1-additive (new optional fields; `competesWith` reuses the **existing**
`Reference` pattern → no widening) → **R1.x**, RFC-gated, **OMM-0**. On landing, `replayPriority`
joins the producer-relative + snapshot enumerations in [Theory & Scope](./theory.md). **Closes
D10**; advances **D8/D4** (the boundary is the consolidation-as-process / continuous-segmentation
half).

**Hits.** ⚡⚡⚡ watts (encode less, small hot set, amortized offline replay) · 🎯 inference (clean
event retrieval, fewer distractors) · capability (continual learning without catastrophic
forgetting). **Open question:** is `competesWith` producer-authored or derivable from
`localDensity` at import — carry both and let the consumer choose.

---

## EP-5 — Temporal-context drift + familiarity sketch (cheap cues, skip retrieval)

**Principle 5 (Temporal Context Model · feeling-of-knowing).** *"A per-record/episode
temporalContext drift vector (timestamps give recency but not cheap similarity-based contiguity);
a producer-level familiarity sketch over entities/cues."*

**(a) temporalContext** is a vector → it rides the Theme-D `Embedding` with a **reserved space
convention**, `"space": "omir:temporal-context"`. Timestamps already give recency; the drift
vector gives cheap *contiguity* (recall the neighbours-in-time of a hit) as a dot product, no
scan (EM-LLM adds exactly this temporal-contiguity stage on top of similarity). No new field —
a documented reserved space + the contiguity-retrieval semantics. **Rides Theme D · R2 · OMM-0.**

**(b) familiarity sketch** is genuinely new: **producer-level** aggregate state OMIR has no home
for. It answers *"do I plausibly hold this?"* before paying for retrieval — and in production the
**skipped** retrievals are often the dominant cost. A negative is authoritative (skip RAG
entirely); a positive is probabilistic. It lives at the **`Bundle`** level (Bundle has no OMM of
its own; it tracks its resources):

```json
"FamiliaritySketch": {
  "type": "object",
  "description": "Producer-level approximate-membership sketch over entities/cues, for metamemory gating: answer 'do I plausibly hold this?' before paying for retrieval (feeling-of-knowing). A negative is authoritative (skip retrieval); a positive is probabilistic. Complements confidence's in-weights-vs-retrieve gate.",
  "properties": {
    "kind":   { "enum": ["bloom", "count_min"], "description": "Sketch family." },
    "domain": { "enum": ["entity", "cue", "content"], "description": "What the sketch is built over." },
    "hashes": { "type": "integer", "minimum": 1, "description": "Number of hash functions k." },
    "bits":   { "type": "integer", "minimum": 1, "description": "Filter width m in bits (bloom) / table width (count-min)." },
    "data":   { "type": "string", "contentEncoding": "base64", "description": "Serialized filter bytes." }
  },
  "required": ["kind", "domain", "hashes", "bits", "data"],
  "additionalProperties": false
}
```

`Bundle` additive field: `"familiaritySketch": { "$ref": "common.schema.json#/$defs/FamiliaritySketch" }`.

**Classification.** temporalContext rides **D · R2**. The sketch is a new optional `Bundle` field
→ **§5.1-additive · R1.x**, RFC-gated, **OMM-0**. (A Bundle-level field, unlike a resource field,
does not interact with any resource's OMM grade.)

**Hits.** ⚡⚡⚡ watts (short-circuit/skip retrieval; cheap temporal cue) · 🎯 inference (contiguity
recall) · capability (temporal-neighbourhood recall). **Open question:** sketch hash-function
identity must be pinned (a named, versioned hash) or a consumer cannot test membership — carry a
`hashAlg` field if EP-5b is balloted.

---

## EP-6 — Chunks, schema typing, and portable spreading activation

**Principle 6 (chunking/expertise · schema · spreading activation).** *"Chunk/template resources
(composedOf) produced by consolidation; schema typing new memories attach to; and — load-bearing
— make the graph spreading-activation-ready by fixing the edge-strength normalization (divergence
D5) so PPR gives the same answer across importers, with salience as seed weights."*

Three deltas; (c) is the one the principle flags **load-bearing**.

**(a) The `Chunk` resource — Templates are reusable Chunks.** A `Chunk` is a consolidation
*product*: a compressed/abstracted unit (`composedOf` the memories/episodes/entities it
consolidates), the episodic→semantic derivation made first-class (divergence D8). A **reusable**
Chunk (`reusable: true`) *is* a Template — a schema/pattern that *new* memories instantiate (via
`MemoryRecord.schemaType`, EP-6b) rather than a one-off abstraction. The two are **one resource
distinguished by the flag, not two resource types** — `Template` is not minted separately. This is
the proposal's only **new core resource type**: it widens the `Reference` pattern and
`Bundle.entry` `oneOf` → **§3.1/§5.2 breaking → R2**, full
[CONTRIBUTING §4](https://github.com/omir-wg/omir-standard/blob/main/CONTRIBUTING.md) RFC
(`spec/rfcs/RFC-<nnnn>-chunk.md`, number TSC-assigned).

**The consolidation *event* is not a resource.** [HANDOFF §5](https://github.com/omir-wg/omir-standard/blob/main/HANDOFF.md)'s
floated `ConsolidationEvent`/`Reflection` re-imports the consolidation *process* that
[semantics.md](./semantics.md) ("what OMIR deliberately does *not* specify") puts out of scope, and
duplicates what Theme E already models. The derivation is carried as a **Theme-E provenance hop on
the Chunk** — `wasGeneratedBy { activity: "consolidation" }` + `wasDerivedFrom` over its
`composedOf` set — never a second resource. Veld concurs *structurally*: its event-sourced journal
`IntentPayload` (`src/intent_log/payload.rs`) is `Remember/Forget/Update/Anchor` — pure memory CRUD
with **no consolidation-event variant**; consolidation lands its output as `Remember`/`Update` of
records, so even Veld's journal models the product, not the event. Draft model:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://omir.io/spec/R2/schemas/Chunk.schema.json",
  "title": "OMIR Chunk (R2 candidate)",
  "type": "object",
  "properties": {
    "resourceType": { "const": "Chunk" },
    "id":   { "$ref": "common.schema.json#/$defs/Id" },
    "meta": { "$ref": "common.schema.json#/$defs/Meta" },
    "content": { "type": "string", "description": "The compressed / abstracted unit — a named template, schema, or expert chunk (MDL: search fewer, denser units)." },
    "composedOf": {
      "type": "array",
      "items": { "$ref": "common.schema.json#/$defs/Reference" },
      "description": "MemoryRecord / Episode / Entity references this chunk consolidates. The episodic->semantic derivation, first-class (divergence D8)."
    },
    "reusable": {
      "type": "boolean",
      "default": false,
      "description": "True => this Chunk is a TEMPLATE: a reusable schema/pattern that NEW memories instantiate (via MemoryRecord.schemaType), not a one-off abstraction. Templates are reusable Chunks, never a separate resource."
    },
    "schemaType": {
      "$comment": "CodeableConcept once Theme A lands; bare string until then.",
      "type": "string",
      "description": "The schema/pattern class this Chunk represents (open vocabulary, Theme A). A MemoryRecord.schemaType equal to this value attaches that memory to this Template (meaningful when reusable=true)."
    },
    "confidence":  { "$ref": "common.schema.json#/$defs/Confidence" },
    "provenance":  { "$ref": "common.schema.json#/$defs/Provenance" },
    "createdAt":   { "$ref": "common.schema.json#/$defs/Instant" },
    "extension":   { "type": "array", "items": { "$ref": "common.schema.json#/$defs/Extension" } }
  },
  "required": ["resourceType", "id", "content", "createdAt"],
  "additionalProperties": false
}
```

Required atomic work items mirror [global-standard Phase 2](./global-standard.md#the-phases)'s
new-resource checklist: author the schema; add to `Bundle.entry.items.oneOf`; widen
`common.schema.json#/$defs/Reference.pattern` to include `Chunk`; extend the validator
`SchemaFiles`/`RESOURCE_TYPES`/registry-walker; register in the JSON-LD `@context`; ship a
migration note (no R1 bundle becomes invalid — older consumers ignore unknown `entry` items).

**(b) `MemoryRecord.schemaType`** — schema-consistent fast integration (Tse et al. 2007): a new
memory attaches to a schema it instantiates. Best as a **CodeableConcept (Theme A)** since
schema-types are an open, per-domain vocabulary → **rides Theme A · R2**, caps at A's grade.

**(c) D5 edge-normalization fix — the load-bearing one.** Today `Relationship.strength` is a
single `[0,1]` scalar; **fan normalization is not representable**, so two importers running
spreading activation / Personalized PageRank over the same exported strengths get **different
answers** — the normalization that makes a strength *mean* something is producer-private. HippoRAG
shows single-pass PPR multi-hop is 10–20× cheaper and 6–13× faster than iterative retrieval *only*
if the weights are portable. `Relationship` additive fields:

```json
"reverseStrength": {
  "$ref": "common.schema.json#/$defs/UnitInterval",
  "description": "Backward association weight P(from|to); asymmetric counterpart to 'strength' = P(to|from). Resolves the symmetric-scalar half of D5 without two independent edges."
},
"normalizedStrength": {
  "$ref": "common.schema.json#/$defs/UnitInterval",
  "description": "Fan-normalized strength: within a source Entity's outgoing edge set sharing one 'normalization', these values sum to <= 1 + epsilon (ACT-R fan S - ln(fan): source activation conserved and divided). THIS is the portable spreading-activation weight; raw 'strength' is producer-private. MUST co-occur with 'normalization' (dependentRequired)."
},
"normalization": { "$ref": "common.schema.json#/$defs/EdgeNormalization" }
```

```json
"EdgeNormalization": {
  "type": "object",
  "description": "Declares the regime that makes 'normalizedStrength' portable. Without it, PPR/spreading-activation diverges across importers (divergence D5).",
  "properties": {
    "scheme": { "enum": ["fan", "softmax", "none"], "description": "fan = source activation divided among associates; softmax = exp-normalized; none = raw (not portable)." },
    "over":   { "enum": ["source", "target"], "description": "Normalized over a node's outgoing (source) or incoming (target) edge set." }
  },
  "additionalProperties": false
}
```

**Seed weights.** With `normalizedStrength` + `normalization` present, PPR is fully specified
across importers when seeded by `Entity.salience` (the documented seed-weight convention) —
salience as seed, fan-normalized edges as the transition matrix, one pass.

**Validator rule — fan-normalization is enforced, not producer-asserted** (new
`Check::GraphNormalization`, codes E250/E251; verified and sapper-hardened):

- **E250 — fan-normalization overflow (error).** For each Entity *E*, partition its edges by
  `(normalization.scheme, normalization.over)`. Within each partition where `scheme ∈ {fan,
  softmax}`: the sum of `normalizedStrength` over edges with `from = E` (`over: source`) — or
  `to = E` (`over: target`) — **MUST be ≤ 1 + ε** (ε = 1e-6, rounding-tolerant, per
  global-standard's `aggregateCredibility` precedent). Edges with `normalization` absent or
  `scheme: none` are excluded; `reverseStrength` is excluded (asymmetry hint, not part of the fan
  sum).
- **E251 — `normalizedStrength` without `normalization` (error).** Also enforced in-schema via
  `dependentRequired: { normalizedStrength: ["normalization"] }`. Without the regime the value is
  unportable — the exact D5 defect the field cures.

*Why it survives the sapper pass:* **upper-bound only** (a partial export's subset of fan weights
sums to ≤ the full sum ≤ 1, so `≤ 1+ε` holds on partial graphs; a lower bound `≈ 1` is deliberately
NOT checked); **partitioned** (mixed regimes never cross-contaminate the sum); **no retroactive
invalidation** (new R2 fields, the rule ships *with* them — §5.2-clean, not a tightening of an
existing field); **decidable & cheap** (pure function of the closed-world bundle, O(edges) group-by
on `from`/`to`, no new index — a fifth `Check` variant beside
`{Structural, ReferenceIntegrity, VersionPresence, Profile}`, mirroring global-standard's
`Check::Attestation`); **not a silent hole** (dodging via `scheme: none` self-labels the value
non-portable — honest degradation, not a bypass). Final code numbers are TSC-assigned.

**Classification.** Chunk (+`reusable` Templates) = **R2 · OMM-0** (§4 RFC, `Reference`-widening).
`schemaType` rides **A · R2**. Edge-norm fields = §5.1-additive (no widening) → **R1.x**, RFC-gated,
**OMM-0**, on `Relationship` (OMM-3), **plus the `Check::GraphNormalization` validator variant
(E250/E251)**. **Closes D5**; advances **D8**.

**Hits.** ⚡⚡ watts (fewer, denser units; single-pass vs iterative multi-hop) · 🎯 inference
(portable multi-hop/transitive) · capability (abstraction, expertise).

---

## EP-F — VSA / HRR structural code (frontier; lowest priority, highest ceiling)

**Frontier (Vector Symbolic Architectures · Holographic Reduced Representations · HDC).** An
optional VSA structural code — bind (circular convolution) + bundle (superpose) a whole relational
structure into one low-precision hypervector, cleanup via an item memory — gives OMIR
**analogical/compositional** retrieval in cheap vector ops. Low-precision and edge-friendly: a
natural fit for the `.omirb` robotics profile. **Speculative** — propose **extension-first** (no
RFC), under a WG/vendor URL, promotable to a reserved Theme-D `Embedding` space once a second
implementer exercises it:

```json
{
  "url": "https://omir.io/spec/R2/ext/vsa-code",
  "valueJson": {
    "space": "vsa", "op": "hrr", "dims": 10000, "dtype": "int8",
    "vector": "<base64 or array>", "cleanupRef": "Entity/item-memory"
  }
}
```

**Classification.** **Extension now** (prefer-an-extension-over-an-RFC, the 80/20 rule); promotable
to a Theme-D `Embedding` space at **R2+ · OMM-0** when field-exercised. Ties to the `.omirb`
binary profile (low-precision bytes). **Hits.** capability (compositional/analogical recall) ·
⚡ watts (low-precision edge ops); 🎯 neutral. No divergence — pure capability frontier.

---

## Overlook — sequence

| Step | EPs | Release · OMM | Breaking? | Unlocks |
|---|---|---|---|---|
| 1 | **EP-1** info-content · **EP-4** replay/interference · **EP-6c** edge-norm | **R1.x** · OMM-0 | No (additive optional fields) | The watt levers that need no new object: store-less, prune-by-interference, portable PPR. **Closes D5 & D10.** |
| 2 | **EP-5b** familiarity sketch · **EP-4a** boundary | **R1.x** · OMM-0 | No | Skip-retrieval gating; segmentation cue. |
| 3 | **EP-2 / EP-3 / EP-5a / EP-F** (all on the Theme-D `Embedding`) | **R2** · OMM-0 | Rides Theme D's R2 line | Efficiency-bearing codes: Matryoshka, sparse, drift, VSA. |
| 4 | **EP-6a** Chunk · **EP-6b** schemaType | **R2** · OMM-0 | **Yes** (new resource / CodeableConcept) | Consolidation product + schema attachment. Needs Themes A & E. |

## Overlook — dependencies

```
Theme D (neutral Embedding) ──► EP-2 (matryoshka), EP-3 (sparse), EP-5a (drift space), EP-F (vsa)
Theme A (CodeableConcept)   ──► EP-6b (schemaType), EP-4a boundaryReason (promotion)
Theme E (PROV wasGeneratedBy)──► EP-6a Chunk's consolidation *event* (vs a 2nd resource)
Theme I (ExternalReference)  ──► EP-3 (index points at remote content)
EP-1 reconstructable         ──► EP-2 verbatim-eviction / EP-3 lazy content load (compose)
P2 (a second implementer)    ──► OMM-2 anywhere; required for the Embedding-code interop tests
```

The **critical path is Step 1** — three pure-additive R1.x field sets that need no new object, no
`Reference` widening, and no other theme, yet **close the two divergences this track inherits
(D5, D10)** and land the biggest watt lever (EP-1). Steps 3–4 are gated on Theme D / A / E landing
first, and the cross-store *value* of the efficiency-bearing codes (like all interchange value)
needs the [global-standard P2 second implementer](./global-standard.md#breakers--adversarial-stress-test-3-passes).

## Definition of done (per EP)

A proposal reaches its stated OMM when, for each field it lands: the **schema** and the
**version-aware reference validator** support it; every new `ResourceType/id` reference field is in
the registry-driven reference walk with a dangling-ref negative fixture in `examples/invalid/`
(EP-4 `competesWith`, EP-6a Chunk refs); and `UnitInterval` fields (EP-1 `novelty`, EP-4
`replayPriority`/`needProbability`, EP-6 `reverseStrength`/`normalizedStrength`) are in the CR-7
range check. EP-6c additionally ships a worked **PPR golden-file** proving two importers produce
the same ranking from the same `normalizedStrength` + `salience` seeds — the portability claim made
testable, the same discipline [global-standard](./global-standard.md#definition-of-done-per-phase)
applies to the RDF and attestation vectors.

## Process

None of this is unilateral. Every change here is a **candidate** — an RFC, debated by the Working
Group and balloted ([CONTRIBUTING](https://github.com/omir-wg/omir-standard/blob/main/CONTRIBUTING.md), [GOVERNANCE](https://github.com/omir-wg/omir-standard/blob/main/GOVERNANCE.md)) —
entering at **OMM-0/1** and earning maturity through *independent* implementation. The efficiency
framing changes the *motivation* (watts, not just interchange), not the gate.
