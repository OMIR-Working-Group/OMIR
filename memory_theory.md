# OMIR — Memory Theory and Its Divergence from the Literature

> **What this is.** A handoff that makes OMIR's *theory of memory* explicit. The
> normative spec ([`spec/src/semantics.md`](spec/src/semantics.md),
> [`principles.md`](spec/src/principles.md)) says *what fields mean over time*; this
> document says *which theory of memory those fields commit to*, traces each choice to
> its intellectual lineage, and maps — honestly — where OMIR aligns with, lags, or
> contradicts the cognitive-science and current AI-agent-memory literature.
>
> **Why it exists.** The adversarial review (this session) found that OMIR's hardest
> critiques are *theory* critiques: "state, not algorithm" leaks because the *choice* of
> first-class state is itself an opinionated theory of memory. This document is the
> answer to that — it names the bets so they can be defended or revised, not discovered
> by a hostile reader. It is the theory companion to
> [`global-standard.md`](spec/src/global-standard.md) (the schema roadmap) and
> [`REMEDIATION.md`](REMEDIATION.md) (today's defects).
>
> **Status: non-normative.** Nothing here changes conformance. It records analysis and
> *recommends* spec/scope changes; it does not make them. Grade/governance changes remain
> gated per [`REMEDIATION.md`](REMEDIATION.md) (TSC + Veld-sync + human sign-off).
>
> **How to read it.** §1 consolidates OMIR's implicit theory. §2 maps the lineage. §3 is
> the core: twelve divergences (Dn), each cross-referenced to the adversarial-review
> finding it grounds (R#). §4 is where OMIR is *ahead*. §5 is the unifying diagnosis. §6
> maps every divergence to a remediation vehicle. §7 is a peer-system comparison.

---

## 1. The theory OMIR actually encodes

Pulled together from [`semantics.md`](spec/src/semantics.md),
[`principles.md`](spec/src/principles.md), the Overview "Heritage" section, and the
schemas, OMIR R1 commits — implicitly but unmistakably — to **eight** theoretical
positions. Stated plainly so they can be argued with:

1. **Memory is declarative, item-based, and externalizable.** The atom is a
   `MemoryRecord`: a discrete, human-readable proposition with metadata. Memory is a
   *collection of records*, not a process, a policy, or a weight matrix.
2. **Belief is Bayesian and should be carried as a distribution.** `confidence` is
   `Beta(α, β)` + a `calibrated` point estimate, not a bare float.
3. **Forgetting is rational and adaptive, parameterized by a half-life.** `decay`
   (`halfLifeHours`, `lastAccess`, `accessCount`, `anchored`) treats forgetting as a
   *feature*: retrievability falls with time and disuse, rises with access, and is
   floored by `anchored`.
4. **Memory consolidates through discrete tiers.** `tier`
   (`working → session → longterm → archive`) is a position in a consolidation hierarchy,
   driven by age × importance × access.
5. **Knowledge is a graph with Hebbian-weighted, directed edges.** `Entity` nodes,
   `Relationship` edges whose `strength` is a synaptic weight that rises with
   co-activation and decays with disuse; references enable spreading activation.
6. **Salience is a stored gravitational/activation quantity.** `Entity.salience` (with
   `mentionCount`, `properNoun`, `lastSeenAt`) is "how strongly an entity pulls on
   retrieval," stored as a scalar.
7. **Memory is bi-temporal and admits prospective intentions.** `eventTime` vs
   `createdAt`; `validUntil` / `validAt` / `invalidatedAt`; `experienceType: "intention"`
   for future-directed memory.
8. **Provenance and source credibility modulate trust.** `provenance`
   (`source`, `sourceType`, `credibility`, `externalId`) lets a ranker weight memory by
   the trustworthiness of its origin.

Taken together these are not eight neutral fields. They are a coherent, recognizable
school of thought: the **rational-analysis / ACT-R / complementary-learning-systems**
lineage of memory — filtered through one production engine (Veld) and serialized.
[`global-standard.md`](spec/src/global-standard.md) admits the provenance ("its schemas
were derived from one production memory engine"); this document names the *theory* that
came with them.

---

## 2. The intellectual lineage

Each OMIR mechanism has a traceable origin in cognitive science, and a counterpart in the
2023–2026 agent-memory literature. The middle column is where the idea comes from; the
right column is who else builds on it now.

| OMIR mechanism | Cognitive-science lineage | Current AI-systems analog |
|---|---|---|
| `MemoryRecord` as atom | Declarative memory; Tulving's episodic/semantic split (1972) | Generative Agents' "memory stream" observations; Mem0 extracted "memories" |
| `confidence` = Beta(α,β)+point | Bayesian models of cognition; calibration/metacognition | Rare — most systems store no calibrated belief at all |
| `decay` (half-life + access) | Ebbinghaus (1885); Anderson & Schooler rational analysis (1991); ACT-R base-level activation | Generative Agents' exponential recency score; most use append-only logs |
| `anchored` floor | Bjork & Bjork *New Theory of Disuse* (1992) — storage strength | Letta "core memory" (pinned, in-context blocks) |
| `tier` (4 levels) | Atkinson–Shiffrin modal model (1968); Baddeley WM; CLS hippocampus→neocortex (McClelland 1995) | Letta/MemGPT core/recall/archival; CoALA working + long-term |
| `Relationship.strength` (Hebbian) | Hebb (1949); Collins & Loftus spreading activation (1975); ACT-R associative strength | Zep/Graphiti edges; A-MEM note links; HippoRAG associative indexing |
| `Entity.salience` | ACT-R activation; attention/arousal; emotional salience (LaBar & Cabeza 2006) | Generative Agents' LLM "importance" score (1–10) |
| `eventTime`/`validUntil`/`invalidatedAt` | Source/temporal memory; DB bitemporality (Snodgrass) | Zep/Graphiti's four-timestamp bitemporal edges |
| `experienceType: "intention"` | Prospective memory (Einstein & McDaniel 1990) | Almost universally absent in agent memory |
| `provenance.credibility` | Source-monitoring framework (Johnson, Hashtroudi & Lindsay 1993) | Rare; Zep carries some source metadata |
| `Episode` → derived resources | Encoding→consolidation→retrieval; episodic backbone | Generative Agents' reflection; A-MEM's note generation |

**Reading of the table.** OMIR sits squarely in the *rational analysis of memory*
(Anderson & Schooler): a memory's strength tracks the statistics of the environment —
recency and frequency predict future need, so they govern retrievability. OMIR's `decay`
block is, almost exactly, a serialization of ACT-R base-level activation *inputs*
(`lastAccess` = recency, `accessCount` = frequency) — but with one parameterization choice
(half-life) standing in for ACT-R's power-law sum. That single substitution is the seed of
the most important divergence (D1).

---

## 3. Where OMIR diverges from the literature

Each divergence states: what the literature holds, what OMIR does, *why* it diverges, the
cost, and the review finding it grounds. They cluster into four families (§5).

### D1 — Exponential half-life vs. power-law forgetting  *(R-review #2)*
**Literature.** Forgetting is robustly **non-exponential**. Wixted & Ebbesen (1991) and
Wixted (2004) find power-function (or power-like) retention; Anderson & Schooler (1991)
show environmental need-probability decays as a power law of time, which ACT-R encodes as
`B_i = ln(Σ_j t_j^{-d})` — a *sum of power-law traces*, not a single half-life.
**OMIR.** Standardizes one scalar, `decay.halfLifeHours` — an exponential
parameterization — while [`semantics.md §2`](spec/src/semantics.md) explicitly concedes
real forgetting is "closer to a power law."
**Why it diverges.** Half-life is the simplest portable parameter and matches *AI
engineering* practice (Generative Agents also use exponential recency, `0.99^hours`); but
it cannot represent a power-law, two-component (ACT-R), or multi-timescale curve faithfully
— it can only approximate one at a single point.
**Cost.** A power-law / ACT-R / spacing-aware engine that exports to OMIR loses its decay
*shape*; the importer reconstructs a different forgetting trajectory. The one decay
parameter the format blesses is the one the field's own evidence argues against.
**Nuance.** The deep flaw is *parameterization rigidity* (only a half-life), not exponential
per se. Carrying a named decay model + parameters would close it (D1 → §6).

### D2 — Single retrievability scalar vs. storage/retrieval-strength duality  *(R #8)*
**Literature.** Bjork & Bjork's *New Theory of Disuse* (1992) separates **storage
strength** (how well-learned; monotonic, never decreases) from **retrieval strength**
(current accessibility; waxes and wanes). The dissociation explains the spacing effect, the
testing effect, and why "forgotten" material relearns fast.
**OMIR.** Collapses both into one retrievability notion (`halfLifeHours` + `lastAccess`),
with `anchored` as a crude binary storage-strength *floor* and `accessCount` as a partial
frequency proxy. The two-factor structure is unrepresentable.
**Cost.** OMIR cannot say "deeply learned but currently inaccessible" — exactly the state
that predicts fast relearning and is central to spaced-repetition memory. A snapshot of
retrieval strength alone (D2 is the theoretical root of the *stale-snapshot* problem, R #8):
the exported value ages under dynamics the file doesn't carry.

### D3 — Calibrated confidence: distribution good, composability overstated  *(R #10)*
**Literature.** Keeping belief as a *distribution* is well-supported (confidence ≠ accuracy;
calibration curves; metacognition). But **opinion pooling / Bayesian updating** is explicit
that you may only *add* evidence counts when observations are **independent**. Merging
correlated evidence by summing α/β double-counts and produces overconfident posteriors.
**OMIR.** Models `confidence` as `Beta(α,β)` (ahead of the field — good) but
[`semantics.md §1`](spec/src/semantics.md) claims merging α/β across stores yields "a
coherent posterior, which averaging two floats can never do." When two stores hold memories
derived from the *same upstream source* — the common case in interchange — the independence
assumption fails. Separately, the `calibrated` "damp toward 0.5 when evidence is thin" rule
has **no specified function** and no literature basis (it is ad hoc shrinkage), so two
producers emit different `calibrated` for identical α/β, defeating "a consumer that only
reads `calibrated` behaves correctly."
**Cost.** The headline composability selling point is valid only under an assumption memory
merging routinely violates; the portable point estimate isn't actually portable.

### D4 — Discrete tiers vs. continuous activation  *(R #2)*
**Literature.** The Atkinson–Shiffrin "boxes" model (1968) is the *historically superseded*
view. Modern accounts — Cowan's embedded-processes (1999), ACT-R activation, Baddeley —
treat working memory as the **activated portion of long-term memory**, a continuous
quantity, not a separate store.
**OMIR.** Reifies a 4-value discrete `tier` enum — the box model the field moved past.
**Why it diverges.** AI *engineering* favors discrete tiers (Letta/MemGPT core/recall/
archival map to context/disk/cold storage; the boundary that matters is "in the context
window or not"). So OMIR matches an engineering convention even as it diverges from the
cognitive science.
**Cost.** Tier-less systems (the dominant vector-store pattern — Mem0, most RAG memory) rank
by a *continuous* recency/importance/relevance blend and have no tier to emit; a continuous
`activation` would have been both more faithful and more general. Systems with a *different*
tier taxonomy (ACT-R declarative/procedural; 2-tier; 7-tier) map lossily.

### D5 — Symmetric scalar edge vs. directed, asymmetric, fan-normalized association  *(R #2)*
**Literature.** Associative strength is **directional and asymmetric** — P(B|A) ≠ P(A|B) —
and, crucially, **normalized by fan**: ACT-R's fan effect (Anderson) divides a source's
activation among its associates (`S_ki = S − ln(fan_k)`). Spreading activation only works
because source activation is *conserved and divided*.
**OMIR.** `Relationship.strength` is one `[0,1]` scalar per directed edge. Asymmetry is
expressible only as two independent edges; **fan normalization is not representable at all**
([`semantics.md §4`](spec/src/semantics.md) defers asymmetric/LTP machinery to extensions).
**Cost.** Two importers running spreading activation over the same exported strengths get
different results, because the normalization that makes the strengths *mean* something
(relative to a node's other edges) is producer-private. A raw `strength: 0.88` is not a
portable activation weight.

### D6 — Stored context-free salience vs. cue-dependent, emergent retrieval  *(R #1, R #9)*
**Literature.** What is "salient" or "retrieved" is **cue-dependent**: there is no
context-free importance. Tulving's encoding specificity (1983), and SAM/REM-style sampling
models, make retrieval a function of the *match between cue and trace*. Generative Agents
make this concrete — retrieval = recency × importance × **relevance**, and relevance is
computed *at query time* against the query embedding; only importance is stored.
**OMIR.** Stores `salience` and `importance` as **context-free scalars**. This (a) makes
them producer-relative and non-comparable across stores (R #9 — my `0.8` ≠ your `0.8`), and
(b) reifies as a *stored property* something the literature treats as *emergent at
retrieval*.
**Cost.** This is the theory-level root of the "incomparable scalars" and "commodity-layer"
critiques: OMIR freezes the output of a query-relative process into a context-free number,
then offers it for cross-system comparison where it has no shared meaning.

### D7 — Declarative-only ontology vs. the multiple-memory-systems taxonomy  *(R #3)*
**Literature.** Squire's taxonomy (1992, 2004) dissociates **declarative** (episodic +
semantic) from **non-declarative** (procedural, priming, conditioning) memory — distinct,
neurally separable systems. CoALA (Sumers et al. 2023), the canonical agent-memory taxonomy,
makes the same split first-class: working + long-term **{episodic, semantic, procedural}**,
where procedural = *code / learned functions*.
**OMIR.** Models declarative memory only. With no representation for:
- **procedural / skill memory** (learned tool-use policies, workflows, macros);
- **parametric memory** (LoRA adapters, fine-tuned weights, learned world-models);
- **vector-native memory** as a first-class object (the embedding *is* the memory, yet it
  rides in an extension — R-review #11);
- priming / implicit memory.
**Cost.** A "memory standard" that models one of the field's two top-level memory systems —
and calls itself "agent memory" — is making an unstated scoping claim. CoALA is the exact
yardstick by which a reviewer will measure the omission.

### D8 — Tier as static state vs. consolidation as a transformative process  *(R #1)*
**Literature.** Complementary Learning Systems (McClelland, McNaughton & O'Reilly 1995;
Kumaran, Hassabis & McClelland 2016) holds that consolidation is **transformation** —
hippocampal episodic traces are replayed and re-encoded as neocortical semantic knowledge.
The act of *becoming semantic* is arguably memory's central operation. The AI analog is
explicit and load-bearing: Generative Agents' **reflection** (recursively generating
higher-level insights from recent memories) and A-MEM's **memory evolution** (new notes
update the attributes of old ones).
**OMIR.** Records `tier` as a static position and *explicitly* excludes consolidation
schedules, replay, and "sleep" ([`semantics.md` — "what OMIR deliberately does not
specify"](spec/src/semantics.md)). It can say *where* a memory sits but not *that it was
transformed*; the episodic→semantic derivation survives only weakly via `Episode.entityRefs`
/ `parentId`.
**Cost.** The single most interesting memory behavior in the current literature
(reflection/evolution as a generative process) is exactly what OMIR puts out of scope —
sharpening R-review #1 (OMIR standardizes the commodity layer and punts the differentiator).

### D9 — Stored record vs. reconstructive, reconsolidating trace  *(R #5)*
**Literature.** Memory is **reconstructive, not reproductive** (Bartlett 1932); retrieval
*alters* the trace (reconsolidation — Nader, Schafe & LeDoux 2000; the misinformation
effect — Loftus). A memory is rebuilt, and rewritten, each time it is used.
**OMIR.** Models memory as an essentially immutable stored record with a `version` counter
and temporal *invalidation* — closer to a database row with audit history than a
reconstruction. `accessCount`/`lastAccess` bump retrievability, but **content does not
reconsolidate**. Contrast Mem0 (LLM-driven `UPDATE`/`DELETE` rewrites memories in place) and
A-MEM (notes evolve).
**Cost / counterpoint.** For an *agent* memory format this may be the **right** engineering
choice — auditable, non-confabulating, diff-able records are a feature, not a bug. But it is
a genuine divergence from how memory works, and it interacts directly with the immutability/
erasure tension (R #5): OMIR's "forgetting" is decay-of-retrievability, never deletion.

### D10 — Time-decay forgetting vs. interference  *(R #5)*
**Literature.** The dominant account of *why* we forget is **interference** (retroactive and
proactive competition among similar traces — McGeoch 1932; Wixted 2004), not pure temporal
decay.
**OMIR.** Models forgetting almost entirely as **time-decay + disuse** (`halfLifeHours`,
`lastAccess`). There is no representation of interference — no notion that two similar
memories compete, or that a new memory degrades an old one's retrievability.
**Cost.** OMIR picks the weaker horn of a century-old debate and omits the stronger one
entirely. An engine whose forgetting is interference-driven (most modern similarity-based
stores effectively are, via nearest-neighbor crowding) has no field to record that dynamic.

### D11 — Stored credibility tag vs. the source-monitoring framework  *(R #2)*
**Literature.** Source monitoring (Johnson, Hashtroudi & Lindsay 1993) holds that we do
**not** store a credibility number; source and trust are *attributed* by heuristic decision
processes over qualitative trace characteristics (perceptual detail, cognitive operations),
and are reconstructed — and fallible — at judgment time.
**OMIR.** Stores `provenance.credibility` and `confidence` as numbers — a reasonable
engineering reification, but one that diverges from how the literature says trust is actually
computed (reconstructed, not stored), and that inherits the producer-relative-scalar problem
(D6).
**Cost.** Modest; flagged for completeness. The reification is defensible; the *comparability*
claim across producers is not.

### D12 — Precise instants vs. the imprecision of episodic time  *(R-review temporal point)*
**Literature.** Memory for *time* is notoriously **reconstructed and imprecise** — we infer
"when" from context and landmarks, not a stored timestamp (Friedman 1993). "Sometime last
spring," "before the incident" is the *normal* form of episodic temporal memory.
**OMIR.** Uses precise RFC-3339 instants everywhere and a partial bitemporal model. Peer
systems are already ahead: Zep/Graphiti carries a **full four-timestamp** bitemporal model
(`t_created`/`t_expired` transaction-time, `t_valid`/`t_invalid` valid-time); OMIR is
"bitemporal in spirit" with scattered fields and **cannot express imprecise time at all**.
**Cost.** The format cannot represent the temporal vagueness that characterizes real
episodic memory, and lags a named peer on the bitemporal model it gestures at. Self-flagged
as [`global-standard.md §H`](spec/src/global-standard.md).

---

## 4. Where OMIR is ahead of, or aligned with, the field

Intellectual honesty cuts both ways. Several OMIR choices are *more* principled than the
mainstream of agent-memory systems:

- **Belief as a distribution.** Carrying `Beta(α,β)` is rare and correct; most systems store
  a bare float or no confidence at all. (The *composability* claim overreaches — D3 — but the
  representation is ahead.)
- **Forgetting as first-class portable *state*.** Most systems treat memory as an append-only
  log; OMIR makes decay, anchoring, and tier survive export. The *separation of state from
  algorithm* is a genuine conceptual contribution even where the chosen state is debatable.
- **Provenance + source credibility in the core.** Source-aware trust weighting is largely
  absent from peer systems; OMIR is right to make it core (D11's caveat notwithstanding).
- **Explicit prospective memory** (`experienceType: "intention"`). Future-directed memory is
  almost universally ignored in agent-memory systems; OMIR is essentially alone in modeling it.
- **Invalidate-not-delete temporal model.** OMIR's `invalidatedAt` aligns with Zep/Graphiti's
  edge-invalidation philosophy — a real convergence with the strongest temporal-KG system.
  (This same choice is what fights right-to-erasure — D9/R #5 — so it is a strength and a
  liability at once.)

The pattern: OMIR is **ahead on representing belief, trust, and time**, and **behind on the
dynamics of forgetting, association, and consolidation** — precisely because it standardized
the parts that *serialize* cleanly and deferred the parts that *compute*.

---

## 5. The unifying diagnosis

OMIR encodes a **rational/Bayesian, declarative, item-and-graph theory of memory** — the
Anderson rational-analysis + ACT-R + CLS lineage — serialized from one engine. Its
divergences are not random; they fall into **four families**, each with a different fix
posture:

1. **Right phenomenon, wrong parameterization** (D1 decay shape, D2 single strength, D5
   symmetric/un-normalized edges). The phenomenon is real and the field agrees it matters;
   OMIR just picked a parameterization narrower than the science. **Fixable, additive** —
   carry named models + parameters instead of one blessed scalar.

2. **Reification of emergent quantities** (D6 salience/importance, D11 credibility, parts of
   D3). The literature treats these as *cue-dependent and emergent at retrieval*; an at-rest
   format must freeze them into stored scalars. **Partly inherent** to any serialization —
   mitigate by labeling them producer-relative and (where possible) carrying the inputs, not
   just the output.

3. **Scope omissions** (D7 procedural/parametric memory, D8 consolidation-as-process, D9
   reconstruction, D10 interference). Whole mechanisms the literature considers central are
   absent. **Mixed** — some are deliberate, defensible non-goals (reconstruction/confabulation
   is arguably *correctly* excluded from an auditable record format); some are real gaps
   (procedural memory) that should at minimum be *declared out of scope* rather than silently
   omitted.

4. **The static-snapshot-of-a-dynamical-system problem** (the through-line of D1, D2, D5, D6;
   R-review #8). OMIR stores a frozen state of a process whose *dynamics it deliberately does
   not carry*. Time-dependent values (retrievability, salience, strength) are stale the instant
   they're written, and the consumer has no defined way to age them. **Inherent** to the
   "state, not algorithm" stance — and therefore the one family that cannot be *fixed*, only
   *honestly bounded*.

The single most important corrective is not a schema change. It is to **state OMIR's theory
and scope explicitly** — "OMIR R1 models declarative episodic/semantic memory under a
rational-analysis lineage; it serializes state, not dynamics; stored scalars are
producer-relative" — so that families (2)–(4) read as *deliberate, bounded design positions*
rather than as blind spots a reviewer discovers. That sentence is free; its absence is what
turns defensible tradeoffs into indictments.

---

## 6. Divergence → remediation map

How each divergence should be handled, and through which existing vehicle. "State scope" =
prose/positioning only, no schema change, do-now, no sign-off. "Schema (R2)" = additive
change through the RFC + ballot process. "Bounded non-goal" = document the limit and stop.

| Dn | Family | Posture | Vehicle |
|---|---|---|---|
| D1 decay shape | parameterization | Schema (R2) | New `decayModel` (named curve + params) alongside `halfLifeHours`; ties to nothing existing — propose as a low-OMM addition. |
| D2 storage/retrieval split | parameterization | Schema (R2) or extension family | Optional `storageStrength` vs `retrievalStrength`; or a blessed extension family first. |
| D3 confidence merge + `calibrated` | parameterization | **State scope, do now** | In [`semantics.md`](spec/src/semantics.md): pin or drop the `calibrated` damping function; add the evidence-independence caveat to the composability claim. |
| D4 discrete tiers | parameterization | Schema (R2) | Allow an optional continuous `activation` next to `tier`; keep `tier` for the engineering case. |
| D5 edge asymmetry/fan | parameterization | Schema (R2) | Optional directional/normalized strength fields; folds into the same graph-edge RFC. |
| D6 emergent salience/importance | reification | **State scope, do now** | Label `salience`/`importance`/`strength` *producer-relative*; optional `normalizationRef` so cross-store incomparability is at least *detectable*. Pairs with R-review #9. |
| D7 procedural/parametric memory | scope omission | **State scope now**, schema later | Declare R1 scope = declarative memory (cite CoALA); open an RFC for a procedural/parametric resource at OMM-0 if demanded. |
| D8 consolidation-as-process | scope omission | Schema (R2) | A first-class `ConsolidationEvent` / `Reflection` derivation resource (already floated in [`HANDOFF.md §5`](HANDOFF.md)); captures episodic→semantic transformation + reflection lineage. |
| D9 reconstruction/reconsolidation | scope omission | **Bounded non-goal** | State that OMIR intentionally models auditable records, not reconstructive traces; link to the erasure tension. |
| D10 interference | scope omission | Bounded non-goal (R2 optional) | Acknowledge time-decay-only; an interference signal could ride an extension if a system needs it. |
| D11 stored credibility | reification | State scope | Note credibility/confidence are stored reifications, comparable only within a producer. |
| D12 imprecise/bitemporal time | parameterization | Schema (R2) | Full bitemporal + imprecise instants — already [`global-standard.md §H`](spec/src/global-standard.md); note Zep is the precedent to match. |

**The "do now, free" set** (D3, D6, D7, D9, D11 — all "state scope"/"bounded non-goal") is
exactly the adversarial review's recommendation to *make the bets visible*. None requires a
schema change, a ballot, or Veld sync.

**Efficiency-track cross-reference.** Two of the schema-bearing divergences have an additional,
*efficiency-first* remediation in [`Efficiency & Information-Bearing Codes`](spec/src/efficiency.md):
**D5** (edge asymmetry/fan) → EP-6c, optional `normalizedStrength` + `normalization` fields that
make spreading-activation / PPR portable across importers; **D10** (interference) → EP-4, an
`Interference` block (`needProbability`, `localDensity`, `competesWith`) that reframes D10 from a
"bounded non-goal" to a load-bearing rational-eviction watt lever. Both are §5.1-additive (R1.x).

**Status — landed (this remediation).** The set is now in the spec as a new
[`Theory & Scope`](spec/src/theory.md) page — declarative-memory scope (D7), producer-relative
stored scalars (D6, D11), the snapshot bound on time-dependent values (family 4 / R-review #8),
and records-not-reconstructions (D9) — with the D3 `calibrated` / composability caveats added in
place in [`semantics.md §1`](spec/src/semantics.md), and the page wired into
[`SUMMARY.md`](spec/src/SUMMARY.md). The schema-bearing divergences (D1, D2, D4, D5, D12) and the
governance B-wave remain as tabled above.

---

## 7. OMIR vs. the current agent-memory systems

Where OMIR's commitments land relative to the systems verified this session. ("—" = not
modeled / out of scope.)

| Axis | OMIR R1 | Mem0 | Letta/MemGPT | A-MEM | Generative Agents | Zep/Graphiti | CoALA (taxonomy) |
|---|---|---|---|---|---|---|---|
| Primary object | record + graph | extracted memory (+graph variant) | editable memory block | evolving note | stream observation | KG node/edge | module taxonomy |
| Decay | half-life (exp) state | — (LLM update) | — (agent-managed) | — | exp recency at query | valid-time intervals | n/a |
| Confidence | Beta(α,β)+point | — | — | — | — | — | n/a |
| Tiers | 4-tier enum | flat vector | core/recall/archival | flat + links | flat stream | graph | working+epi/sem/proc |
| Consolidation | static tier (excluded) | LLM ADD/UPDATE/DELETE | agent tool calls | note evolution | reflection | edge invalidation | LEARN action |
| Mutability | versioned, invalidate-not-delete | **mutable (UPDATE/DELETE)** | agent-edited | evolves | append-only | invalidate-not-delete | varies |
| Bitemporal | "in spirit" (partial) | — | — | timestamp | last-access time | **full 4-timestamp** | n/a |
| Procedural mem | **—** | — | partial (tools) | — | — | — | **first-class** |
| Provenance/trust | core `credibility` | source ref | — | — | — | some source meta | n/a |
| Prospective | `intention` | — | — | — | planning (separate) | — | n/a |
| Its niche | **at-rest format** | retrieval engine | agent runtime | memory engine | simulation | memory service | conceptual map |

**What the table shows.** OMIR is the only column that is a *format* rather than an *engine* —
which is the point, and the trap. Every engine column has a *dynamic* (Mem0's mutation,
Letta's agent-management, A-MEM's evolution, Generative Agents' reflection, Zep's
invalidation) that is the system's actual value, and that OMIR by design does not carry. OMIR
is **ahead** of all of them on belief (Beta), trust (credibility), and prospective memory;
**behind** Zep on bitemporality; and **alone in omitting** procedural memory that CoALA — the
field's reference taxonomy — makes first-class. That is the divergence map in one row: OMIR
serializes what these systems *store*, not what they *do*, and its theoretical bets are an
upstream choice of *which stored state is worth a standard*.

---

## References

**Cognitive science (canonical; author-year).** Ebbinghaus (1885) *Über das Gedächtnis*;
Atkinson & Shiffrin (1968) modal model; Tulving (1972) episodic/semantic, (1983) encoding
specificity; Hebb (1949); Collins & Loftus (1975) spreading activation; Baddeley & Hitch
(1974) working memory; Cowan (1999) embedded-processes; Bartlett (1932) reconstructive memory;
Bjork & Bjork (1992) New Theory of Disuse; Anderson & Schooler (1991) rational analysis;
Anderson et al. (2004) ACT-R; Wixted & Ebbesen (1991), Wixted (2004) power-law forgetting /
interference; McGeoch (1932) interference; McClelland, McNaughton & O'Reilly (1995) and
Kumaran, Hassabis & McClelland (2016) complementary learning systems; Squire (1992, 2004)
memory taxonomy; Johnson, Hashtroudi & Lindsay (1993) source monitoring; Einstein & McDaniel
(1990) prospective memory; Nader, Schafe & LeDoux (2000) reconsolidation; Friedman (1993)
memory for time; LaBar & Cabeza (2006) emotional memory.

**Current AI-agent-memory systems (verified this session).**
- Mem0 — [arXiv 2504.19413](https://arxiv.org/pdf/2504.19413); [State of AI Agent Memory 2026](https://mem0.ai/blog/state-of-ai-agent-memory-2026)
- Letta / MemGPT — [Memory Blocks](https://www.letta.com/blog/memory-blocks); [Agent Memory](https://www.letta.com/blog/agent-memory)
- A-MEM — [arXiv 2502.12110](https://arxiv.org/abs/2502.12110); [repo](https://github.com/agiresearch/a-mem)
- Generative Agents — [Park et al. 2023, arXiv 2304.03442](https://ar5iv.labs.arxiv.org/html/2304.03442)
- Zep / Graphiti — [Rasmussen et al. 2025, arXiv 2501.13956](https://arxiv.org/abs/2501.13956); [Graphiti (Neo4j)](https://neo4j.com/blog/developer/graphiti-knowledge-graph-memory/)
- CoALA — [Sumers et al. 2023, arXiv 2309.02427](https://arxiv.org/abs/2309.02427)

**OMIR internal.** [`spec/src/semantics.md`](spec/src/semantics.md),
[`spec/src/principles.md`](spec/src/principles.md),
[`spec/src/global-standard.md`](spec/src/global-standard.md), [`REMEDIATION.md`](REMEDIATION.md),
[`HANDOFF.md`](HANDOFF.md), and the adversarial review (this session).
