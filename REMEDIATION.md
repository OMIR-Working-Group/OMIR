# OMIR — Remediation Plan (R1 artifact hardening)

> **Scope.** These are defects in the **existing R1 artifacts** (spec text, schemas, the
> example corpus, `context.jsonld`, the validator) surfaced by the multi-agent harden review
> of the adoption plan. They are distinct from the forward-looking proposals in
> [`spec/src/global-standard.md`](spec/src/global-standard.md) — these are things that are
> *wrong or fragile today*.
>
> **Status (2026-06-03): A-wave + C1 EXECUTED & VERIFIED. B-wave DECIDED — execution pending
> sign-off. C2 pending.** The mechanical, OMIR-only items (A1–A4, C1) are done and green
> (validator 8/8, site build, link check, RDF-lift). The governance items are **outward-facing
> and require TSC ratification + human sign-off**; the maintainer has now taken the B-wave
> decisions (below). Those decisions **park the one Veld-sync item** — see B3.

## B-wave decisions (recorded 2026-06-03)

- **B1 = Veld-seated interim TSC, opened later.** Add a one-time GOVERNANCE §2.3 bootstrap
  clause: Veld seats a small interim TSC now to unblock balloting, with a **binding commitment
  to expand to a non-Veld majority (honoring the §2.4 one-third cap) once an external
  implementer joins.** *Consequence:* B2/B3 become ratifiable immediately; full neutrality is a
  near-term commitment, not a day-one property — state that trade-off openly in the clause.
- **B2 = OMM-2 means "exercised in real bundles by ≥1 implementation"** (the GOVERNANCE §4.1
  wording becomes canonical). *Execution:* reconcile principles.md §4 (currently the stricter
  *"more than one system"*) and CONTRIBUTING.md §8 (currently collapses OMM-1/2) **down to**
  §4.1. *Consequence:* the bar is low, so the existing OMM-3/4 grades remain defensible
  single-party — which is precisely why B3 can be deferred. *Trade-off to record:* this is the
  weakest second-implementer signal of the three; the honest-risk emphasis (HANDOFF §2) now
  rests on the **OMM-3** discriminator (cross-validated field use), not OMM-2.
- **B3 = defer the re-grade.** No change to the published grades, the example bundles, or
  Veld's emitted `meta.maturity` now. *Consequence:* **the single 🔴 Veld-sync item is parked —
  there is no required Veld change at this time.** Revisit only if a later ballot raises the bar.

**Now unblocked (OMIR-only, no Veld change):** the B1 interim-TSC clause and the B2 ladder
reconciliation across GOVERNANCE §4.1 ↔ principles.md §4 ↔ CONTRIBUTING.md §8. Both are
**governance-charter edits — still ✋ sign-off before I touch those files.** On your go-ahead
I'll draft them as one governance commit for review.

## Legend

| Mark | Meaning |
|---|---|
| 🟢 OMIR-only | No Veld change needed; contained to this repo. |
| 🔴 Veld-sync **required** | Must land in lockstep with a Veld change or Veld/OMIR disagree. |
| 🟡 Veld-touching (deferred) | No change for R1; will require Veld coordination when the R2 item lands. |
| ✅ agent-executable | Mechanical; safe for an agent to do now. |
| ✋ sign-off required | Governance/grade/outward-facing; needs TSC ratification + human approval. |

## Master table (dependency-ordered)

| ID | Finding (today) | Action | Files | Sync | Gate | Sev | Depends on |
|---|---|---|---|---|---|---|---|
| **A1** | Conformance "Document rules (MUST)" is a positional 1–8 list with no stable IDs; every cross-ref is renumber-fragile. | Assign stable anchors **CR-1…CR-8**. | `spec/src/conformance.md` | 🟢 | ✅ | low | — |
| **A2** | Corpus drifts on number form: `{alpha:9,beta:1}` (`encodings.md:36`) vs `{alpha:9.0,beta:1.0}` (`minimal-bundle.omir:82`). | Make examples/docs internally consistent; add a note that `9`/`9.0` are equivalent at rest. (A *canonical* mandate is B-/R2-scope — see Deferred.) | `spec/src/encodings.md`, `examples/minimal-bundle.omir` | 🟢 | ✅ | low | — |
| **A3** | "The four core resources" is asserted but not enumerated (`bundle.md:122`, `validator/README.md:93`); becomes wrong when Agent lands. | Enumerate the set once, explicitly, and reference it. | `spec/src/resources/bundle.md`, `validator/README.md` | 🟢 | ✅ | low | — |
| **A4** | `context.jsonld` overloads one `source` term across `Episode.source`, `Bundle.source`, `Meta.source`, `Provenance.source`; `from`/`credibility` are globally bound. Silently corrupts any RDF lift and blocks PROV/CodeableConcept later. | Split the overloaded terms into distinct, explicitly-typed predicates; **R1 field names/data are unchanged** (mapping-only). Add a Turtle/N-Quads golden-file test to CI. | `spec/R1/context.jsonld`, `scripts/`, `.github/workflows/ci.yml` | 🟢 | ✅ | med | — |
| **B1** | GOVERNANCE §2.3 has **no bootstrap clause for the first TSC**; the balloting body is self-perpetuating and cannot start, so nothing in B-wave is ratifiable. | Add a one-time founding-TSC procedure (≥2 non-Veld seats so the §2.4 cap holds; lazy-consensus confirmation over a fixed window). | `GOVERNANCE.md` | 🟢 | ✋ | high | — |
| **B2** | **Three inconsistent OMM-2 definitions**: GOVERNANCE §4.1 ("≥1 implementation, real bundles"), principles.md §4 ("more than one system"), CONTRIBUTING.md §8 (collapses OMM-1/2). The "honesty brake" is fiction as written. | Reconcile to **one** ladder across all three docs + the §3.4 band table (recommend: OMM-2 = ≥2 *independent* implementations) and define "independent". | `GOVERNANCE.md`, `spec/src/principles.md`, `CONTRIBUTING.md` | 🟢 | ✋ | high | B1 (to ratify) |
| **B3** | Published R1 grades (MemoryRecord **OMM-4**; Entity/Relationship/Episode **OMM-3**) are **unsupportable single-party** under any honest "≥2 independent" gate. Repeated in 4 places **and stamped into the example bundles' `meta.maturity`**. | Re-grade the R1 table (likely → **OMM-1**, "settled in the reference impl; awaiting a 2nd") across all artifacts **and** update emitted `meta.maturity`. | `GOVERNANCE.md` §4.3, `spec/src/principles.md` §4, `CONTRIBUTING.md` §8, `README.md` grade table, `examples/*.omir` | 🔴 **Veld** | ✋ | high | B2 |
| **C1** | Validator's per-field reference walk is hand-coded with a `parentId` special-case (`lib.rs`) and a single-type `E201` tuple that can't model poly-typed refs; brittle and Agent/PROV-blocking. | Replace with a **registry-driven walker** `(field-path → target-type-SET, ref-shape)`, seeded with the exact current closed-world set; `E201` → set-membership. **Behavior-preserving for R1**, guarded by new dangling-`parentId` + dangling-`sourceEpisode` fixtures added *first*. | `validator/src/lib.rs`, `validator/tests/conformance.rs`, `examples/invalid/` | 🟢 | ✅ | med | A1 (CR-5 ref) |
| **C2** | Validator hard-codes a 6-file / 4-type shape (`SchemaFiles`, `RESOURCE_TYPES`, `COMMON_ID` in `schemas.rs`) and runs E300 last; cannot carry >1 release or fail-fast an unknown major. | Make it **version-aware**: parameterize the schema set by declared major, select at parse time, short-circuit unknown majors to one envelope finding. | `validator/src/schemas.rs`, `validator/src/lib.rs` | 🟢 | ✅ | med | C1 |

## Dependency graph

```
A1 ─┐                         (mechanical, no deps — do any time)
A2 ─┤
A3 ─┤
A4 ─┘

B1 (seat founding TSC) ──ratifies──► B2 (reconcile OMM ladder) ──► B3 (re-grade R1 + Veld meta.maturity)  🔴

A1 (CR anchors) ··► C1 (registry walker + guard fixtures) ──► C2 (version-aware validator)
```

Three independent tracks: **A** (mechanical docs/data, no gate), **B** (governance, strictly
serial B1→B2→B3, all sign-off-gated), **C** (validator refactor, C1→C2). A and C can proceed
immediately; B cannot start ratification until B1 seats a TSC.

## Wave detail

### Wave A — mechanical, OMIR-only (execute now)

- **A1 — CR anchors.** Give Conformance's MUST list stable IDs `CR-1…CR-8` so the global-standard
  plan, future RFCs, and the validator can cite rules that survive renumbering. Pure editorial.
- **A2 — number-form consistency.** Normalize the example/docs to one form (recommend the
  decimal-free `9`/`1` in both, matching how integers read), and state that `9` and `9.0` are
  equivalent `number`s at rest in R1. *Do not* impose a canonical-form **mandate** here — that is
  attestation-era (R2) and Veld-touching (see Deferred).
- **A3 — enumerate the core set.** Replace "the four core resources" prose with an explicit list
  (MemoryRecord, Entity, Relationship, Episode) sourced once, so the R2 addition of Agent is a
  one-line, one-place change.
- **A4 — context term split.** Disambiguate the overloaded `source` predicate and reserve
  `from`/`credibility` so the JSON-LD lift is deterministic and the R2 PROV/CodeableConcept work
  isn't blocked. **R1 producers are unaffected** (field names and bytes don't change; only the
  context document's predicate mappings do). Lock it with an RDF golden-file test in CI.

### Wave B — governance reconciliation (TSC-gated; B3 is the Veld-sync item)

- **B1 — founding-TSC bootstrap.** The procedural first domino: no ballot is callable without a
  seated TSC, and §2.3 has no first-TSC clause. ✋ Human decision on the bootstrap mechanism.
- **B2 — reconcile the OMM ladder.** One definition, three documents, plus the §3.4 band table and
  a written "independent implementation" test. ✋ Ballot once B1 seats the TSC.
- **B3 — re-grade R1 (🔴 Veld-sync).** The honest consequence of B2: with one implementer, the
  OMM-3/4 grades are unsupportable and should drop (recommended OMM-1). **This is the one item that
  must move in lockstep with Veld** — see below. ✋ Human sign-off; note the residual adoption-chill
  risk (publishing a "downgrade" must be paired with the maturity-reform narrative, not done bare).

### Wave C — validator correctness & version-awareness (OMIR-only)

- **C1 — registry walker.** A standalone correctness/maintainability win *now* (kills the
  `parentId` special-case, models poly-typed targets, makes adding refs declarative) and the
  prerequisite for Agent/PROV reference checking in R2. **Guard first:** add dangling-`parentId`
  and dangling-`sourceEpisode` fixtures and confirm they fail *before* refactoring, so the
  behavior-preserving claim is proven. Keeps all current R1 verdicts identical.
- **C2 — version-aware validator.** Lets the *same binary* validate R1 and a future R2, and reports
  an unknown major as a single envelope finding instead of a pile of against-R1 errors. Largely an
  R2 enabler; do it when R2 schema work begins.

## Veld synchronization

**Only one item requires a synchronized Veld change: B3 (the re-grade).** Veld is the reference
implementation; its MIF→OMIR adapter **stamps `meta.maturity`** on emitted resources (e.g.
`meta.maturity: 4` on MemoryRecord, `3` on Entity/Relationship/Episode — visible in
[`examples/minimal-bundle.omir`](examples/minimal-bundle.omir)). If OMIR re-grades those types to
OMM-1 but Veld keeps emitting `4`/`3`, every Veld-produced bundle now **overclaims** against the
reconciled ladder — the exact dishonesty the reform exists to remove. So B3 must land as a paired
change:

1. OMIR: re-grade the four artifacts (GOVERNANCE/principles/CONTRIBUTING/README) **and** the
   committed example bundles' `meta.maturity`.
2. Veld: update the MIF adapter's emitted `meta.maturity` constants to the reconciled grades, in
   the same release window.

A WG conformance note should record the paired change so a third party reading either repo sees a
consistent grade.

**Deferred Veld-touching items (🟡 — no R1 action, coordinate at R2):**
- **Canonical number form.** If/when attestation (R2) mandates a shortest-round-trip canonical form,
  Veld's emitter (which serializes `f32` confidence as `9.0`) must normalize. Not an R1 change; A2
  only makes the *examples* consistent.
- **`Episode.source` widening + context.** A4 changes only predicate mappings; when R2 widens
  `Episode.source` to a CodeableConcept, Veld's emitter and the context evolve together.
- **Agent / Reference widening (R2).** Adding the `Agent` resource and widening the `Reference`
  pattern is an R2 producer change Veld opts into; C1/C2 prepare the validator side.

## Execution gating

- **I can do now (on your go-ahead):** A1, A2, A3, A4, and C1 (+ its guard fixtures) — all 🟢 / ✅,
  behavior-preserving, CI-checked. C2 is also OMIR-only but is best sequenced with R2 work.
- **Needs you / the TSC first:** B1, B2, B3 — governance, grades, and the Veld-paired re-grade.
  These are outward-facing; I will not edit GOVERNANCE/grade tables or the published maturity
  numbers without explicit sign-off.

Tell me which wave(s) to execute. A reasonable first cut: **A1–A4 + C1 now** (safe, improves
correctness and CI), and open **B1–B3** as a governance RFC for the human/TSC to ratify with the
Veld team in the loop.
