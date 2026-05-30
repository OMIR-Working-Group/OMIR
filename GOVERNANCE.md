# OMIR Governance

This document defines how **OMIR — Open Memory Interoperability Resources** (pronounced
*"OH-meer"*) is governed: who decides what, how decisions are made, how the specification
matures and is released, and the terms under which contributions are accepted.

OMIR is an open, vendor-neutral, **at-rest** data format for portable AI agent and
cognitive memory — a document standard, not a product and not a wire protocol. MCP and A2A
transport memory; **OMIR *is* the memory at rest.** This governance exists to keep the
standard credible, stable, and free to implement for everyone.

> **Current release:** OMIR R1.
> **Companion documents:** [CONTRIBUTING.md](CONTRIBUTING.md) (how to propose changes),
> [LICENSE-SPEC.md](LICENSE-SPEC.md) (CC-BY-4.0), [LICENSE-CODE.md](LICENSE-CODE.md)
> (Apache-2.0), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## Table of contents

1. [The OMIR Working Group — mission](#1-the-omir-working-group--mission)
2. [Roles](#2-roles)
3. [Decision process — RFC + public ballot](#3-decision-process--rfc--public-ballot)
4. [The OMIR Maturity Model (OMM)](#4-the-omir-maturity-model-omm)
5. [Versioning & release policy](#5-versioning--release-policy)
6. [Deprecation policy](#6-deprecation-policy)
7. [Licensing & its rationale](#7-licensing--its-rationale)
8. [IP & contribution policy (DCO)](#8-ip--contribution-policy-dco)
9. [Amending this document](#9-amending-this-document)

---

## 1. The OMIR Working Group — mission

OMIR is stewarded by the **OMIR Working Group (WG)**, a vendor-neutral group hosted in a
neutral GitHub organization. The Working Group exists to:

- **Develop and maintain** the OMIR specification, JSON Schemas, examples, and reference
  code as an open, implementation-agnostic standard for memory at rest.
- **Keep OMIR vendor-neutral.** No single implementer controls the format. The standard is
  developed in the open, and decisions are made on technical merit, not commercial
  interest.
- **Preserve interoperability.** Every change is judged by whether it helps independent
  systems exchange agent memory faithfully — writing, reading, archiving, diffing, and
  migrating `.omir` documents across vendors.
- **Grade honestly.** OMIR publishes the maturity of each resource type (the OMM, §4) so
  implementers can tell stable surface from experimental surface. The standard does not
  overclaim.
- **Stay complementary, never competing.** OMIR positions itself alongside transport and
  agent-interaction standards (MCP, A2A), not against them. It defines the artifact at
  rest; they move it.

**Convening, not owning.** Veld — the OMIR reference implementation ("Agentic Memory") —
*convenes* the Working Group and contributes the initial specification, schemas, and
tooling. **Veld does not own the standard.** Once governance is established, Veld
participates on equal footing with every other contributor and implementer. The standard's
copyright vests in the Working Group (see §7), and a Veld seat on the Technical Steering
Committee carries no veto and no privileged vote.

**Scope.** The Working Group governs the contents of the `omir-standard` repository: the
specification prose (`spec/`), the JSON Schemas (`schemas/` — the normative ground truth),
the conformance corpus (`examples/`), and the reference code (`validator/`, `generators/`).
It does **not** govern any implementer's product, codebase, or commercial terms.

---

## 2. Roles

OMIR uses three roles. Each is earned through participation, not appointment, and each is
open to anyone regardless of employer.

### 2.1 Contributors

Anyone who proposes a change — an issue, a discussion, an RFC, a schema edit, an example,
documentation, or reference code — is a **Contributor**.

- No prior status is required. The bar to entry is a signed-off contribution (§8) that
  follows [CONTRIBUTING.md](CONTRIBUTING.md).
- Contributors propose; Maintainers and the TSC review and merge.
- Contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md) and the
  schema and process rules in CONTRIBUTING.md.

### 2.2 Maintainers

**Maintainers** are Contributors with merge rights to the repository. They carry the
day-to-day responsibility for the standard's quality.

- **Responsibilities:** triage issues and RFCs; review PRs against the schema style rules
  and the conformance corpus; ensure the reference validator passes over `schemas/` and
  `examples/` before merge; uphold the generated-pages rule (never hand-edit
  `spec/src/resources/`); and shepherd RFCs through ballot.
- **Merge bar:** routine schema and example changes require **two Maintainer approvals**
  (mirroring CONTRIBUTING.md §7). New core resources additionally require the RFC ballot
  (§3). Editorial and tooling fixes may merge with one approval at Maintainer discretion.
- **Becoming a Maintainer:** sustained, high-quality contribution over time, followed by
  nomination by an existing Maintainer or TSC member and confirmation by the TSC. There is
  no employer quota, but the TSC actively guards against any single organization holding a
  Maintainer majority (§2.4).
- **Stepping down / inactivity:** Maintainers may step down at any time. The TSC may move a
  Maintainer to emeritus (read-only) status after a prolonged period of inactivity;
  emeritus Maintainers may be reinstated on request.

### 2.3 Technical Steering Committee (TSC)

The **Technical Steering Committee** is the standard's technical authority and final
decision-making body. It is small, vendor-balanced, and accountable to the community.

- **Responsibilities:** set technical direction; assign RFC numbers; call and adjudicate
  ballots (§3); approve release cuts (§5); approve maturity (OMM) level changes (§4);
  publish and enforce the deprecation policy (§6); confirm new Maintainers; and resolve
  disputes that Maintainers escalate.
- **Composition:** a small odd-numbered seat count (target **3–7**) to avoid ties.
  Members serve fixed, renewable terms (recommended **12 months**). Seats are filled by
  community nomination and confirmation by the sitting TSC.
- **Vendor-neutrality cap:** **no single organization may hold more than one third of TSC
  seats** (rounded down), and no organization holds a veto. This cap is the structural
  guarantee behind "Veld convenes but does not own."
- **Conflict of interest:** members recuse themselves from any ballot in which their
  employer has a material, non-interoperability interest, and recusals are recorded in the
  ballot result.
- **Transparency:** TSC decisions, ballots, and rationale are recorded in public — in the
  relevant RFC, issue, or a published meeting note. The TSC does not make binding technical
  decisions in private.

### 2.4 Vendor-neutrality safeguards (all roles)

To keep OMIR genuinely neutral:

- No organization may hold a majority of Maintainer seats or more than one third of TSC
  seats.
- A change cannot be merged solely by reviewers from a single organization when an
  independent reviewer has requested changes in good faith; escalate to the TSC instead.
- "Convening" confers process responsibility (hosting, scheduling, bootstrapping), **not**
  ownership, veto, or a privileged vote.

---

## 3. Decision process — RFC + public ballot

Substantive changes to OMIR move through an open **RFC (Request for Comments)** followed by
a **public ballot**. The lifecycle status of a resource or feature tracks a three-stage
maturation track — **Draft → Trial Use → Normative** — which is recorded for the affected
resource type as an OMM level (§4).

### 3.1 What requires an RFC

An RFC is required for any change that affects the **normative surface** of the standard,
including:

- Adding or removing a **core resource type** (widens the `Reference` pattern and the
  `Bundle.entry` `oneOf`).
- Changing a resource's **fields, types, enums, constraints, or `required` set**.
- Changing shared definitions in `common.schema.json`.
- Changing a resource's **OMM level** (§4).
- Changing **process or governance** (this document, CONTRIBUTING.md process sections, the
  deprecation policy).

Changes that do **not** require an RFC — and may proceed as ordinary pull requests under
CONTRIBUTING.md — include: registering an `extension[]` (the typed escape hatch never
changes core meaning), adding examples to the conformance corpus, editorial fixes to
schema `description` text, and reference-code maintenance that does not alter conformance.

> **Prefer an extension over an RFC.** OMIR is an 80/20 core plus a typed `extension[]`
> escape hatch. If proprietary or experimental data can ride on `extension[]` without
> changing how the core is interpreted, it should — that is the design working as intended,
> and it needs no RFC. See CONTRIBUTING.md §5.

### 3.2 The RFC lifecycle

```
Idea → Discussion → RFC (Draft) → Public comment → Ballot → Accepted / Rejected / Withdrawn
                                                              │
                                                              └→ merged at a graded OMM level
```

1. **Discussion first.** Float the idea as an issue or discussion so the Working Group can
   confirm it cannot be met by an extension on an existing resource.
2. **Author the RFC** using the skeleton in CONTRIBUTING.md §4 and place it under
   `spec/rfcs/` as `RFC-<nnnn>-<short-slug>.md`. The TSC assigns the number.
3. **Public comment.** The RFC stays open for community comment for a **minimum of 14
   calendar days**. Material revisions during comment restart no clock unless the TSC says
   otherwise, but the TSC should allow enough time for independent review.
4. **Ballot.** When discussion has converged, the TSC calls a ballot (§3.3).
5. **Outcome.** An accepted RFC is merged with its schema and at least one validating
   example, and lands at the OMM level justified in the RFC. A rejected or withdrawn RFC is
   recorded with its rationale so the decision is discoverable later.

### 3.3 Ballot rules

- **Who votes:** the TSC casts the binding ballot. Maintainers and the wider community
  advise through public comment, and the TSC is expected to weigh that input on the record.
- **Quorum:** a ballot requires participation from a **majority of sitting TSC members**.
- **Threshold:** acceptance requires a **two-thirds majority of votes cast** (after
  recusals). A simple majority is insufficient for normative change; this raises the bar
  for anything that ships in the stable core.
- **Recusal:** members with a conflict of interest recuse and are excluded from quorum and
  threshold math for that ballot; recusals are recorded.
- **Transparency:** every ballot records who voted, the tally, recusals, and a one-line
  rationale, in public.
- **Lazy consensus for the routine.** Non-normative changes (editorial, examples, tooling)
  do not require a formal ballot; they merge under the Maintainer review bar in §2.2.

### 3.4 The maturation track: Draft → Trial Use → Normative

A new resource type, or a significant change to one, advances along a maturation track. The
track is the human-readable name for movement up the OMM (§4):

| Stage | OMM band | What it means |
|---|---|---|
| **Draft** | OMM-0 | Newly proposed. Shape may change freely; not safe to depend on. |
| **Trial Use** | OMM-1–2 | Implemented somewhere and exercised, but with limited field experience. Feedback is actively solicited; breaking changes are still possible within a release per the deprecation policy. |
| **Normative** | OMM-3–5 | Real, multi-implementation usage. Stabilizing (3), mature (4), or stable/normative (5). Breaking changes only across releases, per the deprecation policy (§6). |

Advancement up the track is itself an RFC + ballot decision (§3.1): a resource does not get
promoted because its authors are confident, but because field evidence justifies it, and
the TSC has balloted the promotion.

---

## 4. The OMIR Maturity Model (OMM)

OMIR grades stability with the **OMIR Maturity Model (OMM)** — an integer scale **0–5,
assigned per resource TYPE** (not per field, not per release). The level is surfaced in
`meta.maturity` on resources and on each resource's schema/spec page. The OMM exists so an
implementer can tell, at a glance, how much of the standard is safe to build on.

### 4.1 The six levels

| Level | Name | One-line criterion |
|:---:|---|---|
| **0** | **Draft** | Newly proposed; the shape may change freely and is not safe to depend on. |
| **1** | **Trial Use (early)** | Implemented in at least one system; minimal field experience; breaking changes expected. |
| **2** | **Trial Use (proven)** | Exercised in real bundles by at least one implementation; shape settling but not yet cross-validated. |
| **3** | **Established** | Multiple independent implementations and real-world usage; stabilizing, with conservative change. |
| **4** | **Mature** | Broad field experience across implementations; changes are rare and strictly backward-compatible within a release. |
| **5** | **Normative** | Stable and authoritative; breaking changes only across releases, under the deprecation policy (§6). |

### 4.2 Grading rules

- **Honest grading is mandatory.** A level reflects *demonstrated reality*, not ambition or
  marketing. A resource may be widely deployed and still sit at a lower level if it lacks
  independent implementations.
- **Evidence required to raise a level.** No PR may raise a resource's OMM level without
  field evidence (independent implementations, conformant examples, stable usage). Raising
  a level is a normative change and requires an RFC + ballot (§3).
- **Levels can fall.** If field experience reveals a design flaw, the TSC may *lower* a
  resource's OMM level by ballot. Maturity tracks truth in both directions.
- **New resources start low.** Brand-new core resources enter at **OMM-0 or OMM-1** and earn
  their way up.

### 4.3 Current R1 grades

| Resource type | OMM level | Basis |
|---|:---:|---|
| **MemoryRecord** | **4 — Mature** | Broad field experience as the atomic unit of agent memory in the reference implementation; the most exercised resource. |
| **Entity** | **3 — Established** | Real usage and a settled shape; awaiting broader independent implementation. |
| **Relationship** | **3 — Established** | Directed, weighted, Hebbian edge; settled but pending wider cross-implementation. |
| **Episode** | **3 — Established** | Episodic backbone; settled core, narrower deployment than MemoryRecord. |
| **Bundle** (container) | n/a | The `.omir` document container; its maturity tracks the resources it carries rather than carrying an independent OMM level. |

These grades are consistent with CONTRIBUTING.md §8 and the README. Any change to them is a
balloted, normative change (§3).

---

## 5. Versioning & release policy

OMIR uses **semantic, FHIR-style release versioning**: named major releases **R1, R2, R3,
…**. A release is a coherent, frozen snapshot of the normative surface (schemas + spec) that
implementers can target by name.

### 5.1 What may change *within* a release (e.g. within R1)

A release is **additive and backward-compatible** by construction. Within R1, the following
are permitted because they do not invalidate any existing conformant bundle:

- **New optional fields** on existing resources.
- **New `extension[]` registrations** (extensions are the escape hatch; they never change
  core meaning).
- **New examples** in the conformance corpus.
- **New open-vocabulary values** documented in a field's `description` (e.g. a new
  `relationType` string), since open vocabularies already invite new lowercase
  `snake_case` values.
- **OMM level increases** for a resource, backed by field evidence and a ballot (§4) — a
  maturity grade is metadata about stability, not a change to the wire/at-rest shape.
- **Editorial clarifications** to `description` text that do not change meaning.
- **Reference-code releases** (validator, generators) on their own cadence.

**Invariant:** *no change accepted into an existing release may cause a previously valid
`R<n>` bundle to become invalid.* If a proposed change would break an existing bundle, it
does not belong in the current release — it is queued for the next one (§5.2).

### 5.2 What may change only *across* releases (R1 → R2)

Changes that alter or remove existing normative surface are **breaking** and may land only
in a new major release:

- Removing or renaming a field, enum value, or resource type.
- Tightening a constraint or expanding the `required` set in a way that rejects previously
  valid documents.
- Changing the type or semantics of an existing field.
- Any change to `common.schema.json` shared definitions that is not purely additive.

Breaking changes are collected against the next release and shipped together, so
implementers face a small number of well-documented migration boundaries rather than a
stream of surprises. Each new release publishes a **migration note** describing what changed
and how to upgrade bundles produced under the previous release.

### 5.3 Cutting a release

A new release (`R<n+1>`) is cut by **TSC ballot** (§3.3). The release ballot freezes the
schema set, publishes the migration note and the updated OMM grades, tags the repository,
and updates the canonical `$id` / `@context` namespace
(`https://omir.io/spec/R<n+1>/...`). Released schemas under a prior namespace remain
published and resolvable so old bundles stay verifiable indefinitely.

### 5.4 Reference-code versioning

The reference code (`validator/`, `generators/`) is versioned independently using standard
semantic versioning (`MAJOR.MINOR.PATCH`) and states which OMIR release(s) it supports. A
reference-code release never redefines the standard; it implements it.

---

## 6. Deprecation policy

Nothing in a published release is removed without warning. OMIR follows a **deprecate-then-
remove** discipline so implementers always have a migration window.

1. **Mark, don't delete.** To retire a field, enum value, or resource type, it is first
   marked **deprecated** in the schema `description` and the spec page, with the reason, the
   replacement (if any), and the **earliest release in which removal may occur**. The item
   remains fully valid and functional while deprecated.
2. **Minimum one-release notice.** A deprecated item must remain present and valid for **at
   least one full major release** after the release in which it was marked deprecated.
   Nothing is deprecated and removed in the same release.
3. **Removal is a breaking change.** Actual removal happens only across a release boundary
   (§5.2) and only by TSC ballot. The removing release's migration note documents the
   removal and the supported replacement path.
4. **Validators warn before they fail.** The reference validator emits a **warning** for
   deprecated-but-present items and reserves **errors** for items removed in the targeted
   release, so producers can detect and migrate ahead of a hard break.
5. **Old documents stay verifiable.** Because prior-release schemas remain published under
   their original namespace (§5.3), a bundle produced under an earlier release can always be
   validated against the release it declares, even after the item is gone from later
   releases.

This policy is published, not discretionary: the deprecation marker, the notice window, and
the removal release are part of the public record for every retired item.

---

## 7. Licensing & its rationale

OMIR's license is **deliberately split**, and that split is a governance decision, not an
accident.

| Artifact | License | Scope |
|---|---|---|
| Specification prose & JSON Schemas | **CC-BY-4.0** | `spec/`, `schemas/`, JSON-LD `@context`, design notes |
| Examples (conformance corpus) | **CC-BY-4.0** | `examples/**` |
| Reference code (validator, generators) | **Apache-2.0** | `validator/**`, `generators/**` |

See [LICENSE-SPEC.md](LICENSE-SPEC.md) and [LICENSE-CODE.md](LICENSE-CODE.md) for the
authoritative terms.

### 7.1 Why the spec is CC-BY-4.0

**A standard published under a restrictive license is dead on arrival.** For OMIR to be a
genuine interoperability format, anyone — competitor, hobbyist, or hostile fork — must be
free to read it, copy it, quote it, and re-implement it from scratch, including
commercially, with no permission and no fee. CC-BY-4.0 grants exactly that, requiring only
attribution. The spec is, and will remain, free.

### 7.2 Why the reference code is Apache-2.0

The validator and generators are *executable tooling*, where the relevant concerns are
patent peace and clear contribution terms rather than text reuse. **Apache-2.0** carries an
explicit patent grant and well-understood contribution semantics, which is what implementers
need to adopt and ship the reference tooling without legal anxiety.

### 7.3 Decoupled from any implementer's commercial license

The OMIR license is **independent of any implementer's own licensing**, including that of
the reference implementation, Veld. Veld's core ships under a commercial source-available
license (BUSL-1.1); **none of that reaches OMIR.** The standard does not inherit, and must
never inherit, a single vendor's commercial terms. This decoupling is the legal
counterpart to the vendor-neutrality safeguards in §2: just as no organization controls the
TSC, no organization's product license constrains the standard.

> **Rule:** do not copy text or code into this repository under any license other than the
> two above (CC-BY-4.0 for spec/schemas/examples, Apache-2.0 for reference code) without
> prior TSC approval.

---

## 8. IP & contribution policy (DCO)

OMIR accepts contributions under the **Developer Certificate of Origin (DCO) 1.1**
(<https://developercertificate.org/>). There is **no CLA** — the DCO is a lightweight,
auditable sign-off that keeps the barrier to contribution low while protecting the
standard's provenance.

### 8.1 What the sign-off certifies

By signing off a commit you certify, per the DCO, that you wrote the contribution or
otherwise have the right to submit it, and that you submit it under the project's licenses
(§7): **spec and schema changes under CC-BY-4.0, reference-code changes under Apache-2.0.**

### 8.2 How to sign off

Every commit MUST carry a `Signed-off-by` trailer whose name and email **match the commit
author**:

```
Signed-off-by: Jane Doe <jane@example.com>
```

The simplest way is the `-s` flag:

```bash
git commit -s -m "schema(Episode): clarify eventTime vs createdAt"
```

PRs with unsigned commits are blocked until every commit is signed off.

### 8.3 Inbound = outbound

Contributions are licensed to the project on the same terms the project distributes them
(§7). Contributors retain copyright in their work; they grant the project and its users the
CC-BY-4.0 (spec) or Apache-2.0 (code) license to use it. Patent peace for the reference code
flows through the Apache-2.0 grant. No additional rights assignment is requested or
required.

### 8.4 Third-party material

Do not introduce text, schema fragments, or code copied from elsewhere unless its license is
compatible with §7 and its provenance is clearly attributed (and, for anything non-trivial,
approved by the TSC). When in doubt, write it fresh or ask first.

---

## 9. Amending this document

Governance changes are themselves normative and go through the RFC + ballot process (§3):
an RFC under `spec/rfcs/`, a public comment period, and a TSC ballot meeting the two-thirds
threshold (§3.3). Amendments are recorded in the public history so the evolution of OMIR's
governance is as inspectable as the standard it governs.

The OMIR specification, schemas, reference code, and this governance are stewarded by the
vendor-neutral **OMIR Working Group**. Veld convenes the Working Group but does not own the
standard.
