# OMIR — Handoff Brief for a New Agent

> **You are taking over the OMIR project in a fresh workspace.** This document is your
> complete briefing — read it top to bottom before acting. It is written so that an
> agent with **zero prior context** can operate the project correctly. Everything you
> need (decisions already made, what exists, what to build next, what NOT to touch) is
> here. When in doubt, the repository's `schemas/` are the single source of truth.

---

## 0. Your mission, in one breath

OMIR is a new open **standard** — a vendor-neutral, at-rest **data format for portable AI-agent memory**, designed in the mold of HL7 FHIR. The format and its home website (**omir.io**) are the two active tracks. A Cloudflare-native email system for the domain is **designed but backlogged** — do not start it until the standard and website are substantially done.

**Priority order (respect this):**
1. **The Standard** — harden the spec, build the reference tooling, grow the conformance corpus.
2. **The Website** (`omir.io`, on Cloudflare) — host the spec, a validator playground, governance, adopters.
3. **(BACKLOG) Email** — Cloudflare-native architecture decided, recorded in §7. **Do not build it yet.**

---

## 1. What OMIR is, and why it exists

**OMIR = Open Memory Interoperability Resources** (pronounced "OH-meer").

The AI-agent-memory space (2026) is saturated with **products** — Mem0 (raised $24M), MemoryLake ("memory passport"), 6+ separate products literally named "Engram," DeepSeek's "Engram" technique — but there is **no neutral, at-rest *format* standard**. Protocols exist for *transporting* agent context (MCP and A2A, both now under the Linux Foundation Agentic AI Foundation) but they move memory; none defines what memory **is** when written to disk and handed between vendors.

OMIR fills that gap. The strategic analogy is exact:

| FHIR | OMIR |
|---|---|
| **F**ast | **O**pen |
| **H**ealthcare | **M**emory |
| **I**nteroperability **R**esources | **I**nteroperability **R**esources |

FHIR didn't win by being a better hospital — it won by being the neutral format every hospital writes to. OMIR's wedge is identical: don't compete with the memory engines, **be the format they all export to.** "Open" leads the acronym deliberately — it's the one positioning a commercial product can't copy without abandoning its moat.

**Reference implementation:** Veld ("Agentic Memory"), a sibling repo. Veld's internal "MIF" (Memory Interchange Format) becomes *an implementation of* OMIR — the way Epic implements FHIR. OMIR itself is vendor-neutral and must stay that way.

---

## 2. Pinned decisions — DO NOT re-litigate without strong cause

These were researched and settled. Reopening them wastes effort.

- **Name / acronym:** OMIR — "Open Memory Interoperability Resources." Rejected: anything "engram" (6+ live products + DeepSeek) or "mem*/memory-layer/memory-passport" (Mem0/MemoryLake turf).
- **File extensions:** `.omir` (canonical JSON / JSON-LD) and `.omirb` (compact binary profile, CBOR; bincode as an internal sub-profile). Both verified collision-free. **Rejected** `.mf` (Java JAR/MetaFont/Panda3D/FrameMaker) and `.mif` (MapInfo/FrameMaker) — both heavily squatted.
- **Domain:** **`omir.io`** — purchased, hosted on **Cloudflare**. (`omir.org`/`.com`/`.dev`/`.net`/`.co`/`.app` are all taken; `omir.io` was double-confirmed available via NS + RDAP. Exact match, and `.io` is the protocol-native TLD — cf. `modelcontextprotocol.io`.) Every `omir.org` placeholder in the repo has already been rewritten to `omir.io`.
- **GitHub org:** `omir-wg` — already wired into `spec/book.toml` (`git-repository-url` + `edit-url-template` → `https://github.com/omir-wg/omir-standard`). The org **name is decided**; only its public **registration** is pending human approval. Don't invent a different org.
- **Positioning:** an **at-rest data format**, NOT a wire protocol and NOT a product. Frame relative to MCP/A2A as **complementary**: "MCP/A2A transport memory; OMIR *is* the memory at rest." Pitch *into* the Linux Foundation Agentic AI Foundation, never against it.
- **Data model:** FHIR-style. Everything is a **Resource**; resources link by typed reference `ResourceType/id`; an **80/20 core** plus a typed `extension[]` escape hatch for proprietary data; constrainable via **Profiles**.
- **Maturity model:** **OMM** (OMIR Maturity Model), integer levels 0–5 per resource type, surfaced in `meta.maturity`. **Grade honestly. Never overclaim stability.**
- **Licensing (critical):** the **spec + schemas are CC-BY-4.0**; the **reference code (validator, generators) is Apache-2.0**. Deliberately decoupled from Veld's BUSL-1.1 core — a standard under a restrictive license is dead on arrival.
- **Honest risk (keep front of mind):** standards die without a **second implementer**. Securing an external adopter is more important than any feature.

---

## 3. Current state of the repository

**Location:** `c:\Repositories\Portll\omir-standard` (a sibling of the Veld repo, kept separate so it stays license-clean). **Not yet `git init`'d.**

```
omir-standard/
├─ README.md                     project front page (hero, FHIR table, layout, quickstart)
├─ HANDOFF.md                    ← this file
├─ POSITIONING.md                one-page positioning / announcement memo
├─ GOVERNANCE.md                 working group, RFC→ballot process, full OMM 0–5 table, versioning, license rationale
├─ CONTRIBUTING.md               how to propose a Resource/Extension, schema style, DCO flow
├─ CODE_OF_CONDUCT.md            Contributor Covenant v2.1
├─ LICENSE-SPEC.md               CC-BY-4.0 (spec, schemas, examples)
├─ LICENSE-CODE.md               Apache-2.0 (reference code)
├─ .gitignore
├─ schemas/                      ← SINGLE SOURCE OF TRUTH (JSON Schema draft 2020-12)
│   ├─ common.schema.json        Meta, Reference, Instant, UnitInterval, Confidence, Decay, Provenance, Extension
│   ├─ MemoryRecord.schema.json  (OMM-4)
│   ├─ Entity.schema.json        (OMM-3)
│   ├─ Relationship.schema.json  (OMM-3)
│   ├─ Episode.schema.json       (OMM-3)
│   └─ Bundle.schema.json        the .omir document container (no OMM grade — it's the envelope)
├─ examples/
│   └─ minimal-bundle.omir       conformant document exercising all 5 resources + refs + an extension
├─ spec/                         mdBook specification
│   ├─ book.toml
│   ├─ R1/context.jsonld         JSON-LD 1.1 @context (~56 terms → https://omir.io/ns#)
│   └─ src/
│       ├─ SUMMARY.md            mdBook TOC
│       ├─ overview.md  principles.md  encodings.md  extensions.md  profiles.md  conformance.md
│       └─ resources/            memoryrecord.md  entity.md  relationship.md  episode.md  bundle.md
├─ generators/README.md          DESIGN DOC ONLY — schema→spec-page generator (omir-gen, Rust, Apache-2.0). NOT built.
└─ validator/README.md           DESIGN DOC ONLY — omir-validate CLI (Rust, Apache-2.0). NOT built.
```

**Status by component:**
- ✅ **Schemas** — complete and valid (all 6 parse; derived directly from Veld's real `Memory`, `EntityNode`, `RelationshipEdge`, `EpisodicNode` types). These are authoritative.
- ✅ **Example** — `examples/minimal-bundle.omir` is conformant and parses.
- ✅ **Spec prose** — drafted (overview, principles, encodings, extensions, profiles, conformance + 5 resource pages). Resource pages are written in "generated" style and carry a "do not hand-edit" banner.
- 🟡 **Validator & generator** — **design docs only.** The READMEs describe tools that **do not exist yet**. Building them is a priority-1 task.
- ❌ **git / CI / website** — none yet.

**Verify the repo yourself on day one:** validate every JSON artifact parses, and confirm zero `omir.org` strings remain (all should be `omir.io`).

---

## 4. The standard, in enough depth to work on it

**Core resources (R1):**
- **MemoryRecord** (OMM-4) — the atomic unit: a remembered experience, plan, prompt, or learning. Carries `content`, `kind`, `experienceType`, `tier` (working→session→longterm→archive), `createdAt`/`eventTime`, `importance`, Bayesian `confidence` (α/β + calibrated), `decay` (+ `anchored`), `provenance` (+ source credibility, `externalId`), `entityRefs[]`, `parentId`, `validUntil` (temporal invalidation), `version`, `extension[]`.
- **Entity** (OMM-3) — a named thing (person/org/location/concept/technology/product/skill/keyword/project). `salience`, `properNoun`, `mentionCount`, `attributes`, `labels[]`.
- **Relationship** (OMM-3) — directed, weighted (Hebbian `strength`) edge between two Entities; `relationType` (open vocab), temporal `validAt`/`invalidatedAt`, `sourceEpisode`.
- **Episode** (OMM-3) — the bounded raw experience other resources derive from; `source` (message/document/event/observation), `eventTime` vs `createdAt`, `entityRefs[]`.
- **Bundle** — the `.omir` document itself: `{ resourceType:"Bundle", omirVersion:"R1", entry:[...] }`, JSON-LD compatible via optional `@context`.

**Cross-cutting mechanisms:** typed references (`ResourceType/id`, must resolve within the bundle); `extension[]` (namespaced, consumers MUST ignore unknown); `meta` (omirVersion/source/maturity/timestamps); Profiles (`omir-coding-agent`, `omir-robotics`, `omir-personal-assistant`); two lossless encodings (`.omir` JSON canonical, `.omirb` binary).

**Heritage features the format must faithfully carry** (from Veld): calibrated Bayesian confidence, multi-time-scale decay + anchoring ("better forgetting"), tiered memory, Hebbian edge strength, temporal invalidation, provenance + source credibility, prospective memory (`experienceType: "intention"`), entity salience.

---

## 5. TRACK 1 — The Standard (priority 1). Concrete next steps

1. **Build `omir-validate`** (Rust, Apache-2.0) per `validator/README.md`: structural validation against the schemas + **reference integrity** (every `ResourceType/id` resolves) + `meta.omirVersion=="R1"` + optional profile checks; emit a conformance report (JSON + human). Gate the "Powered by OMIR" badge on it.
2. **Build `omir-gen`** (Rust, Apache-2.0) per `generators/README.md`: render `spec/src/resources/*.md` field tables **from** `schemas/`. Add a CI check that fails if committed pages drift from a fresh generation. (Mirrors Veld's `gen-module-index.rs` discipline.)
3. **Grow the conformance corpus** in `examples/` — minimal + full per resource, plus deliberately **invalid** fixtures (dangling ref, bad enum, score out of range) the validator must reject.
4. **Consider R1 additions at low OMM** (don't overreach): `ContextBlock` (Letta-style mutable agent state), `Intention` as a first-class resource, `Provenance`/`ConsolidationEvent` as resources. Propose via the RFC process in `CONTRIBUTING.md`; grade them OMM-0/1.
5. **`git init`**, push to the **`omir-wg`** GitHub org (already wired into `spec/book.toml`; see §2), wire CI: JSON-schema self-validation, mdBook build, and a **link checker** (see gotchas §8).

**Toolchain (pin these):** Rust **edition 2021** + a committed `rust-toolchain.toml`; schema validation via the **`jsonschema`** crate (draft 2020-12, with local `$ref` resolution across `schemas/`); CI installs **mdBook 0.4.x** for the spec build. `omir-validate` and `omir-gen` are separate Apache-2.0 crates, each with its own `[workspace]` (kept out of Veld's workspace).

---

## 6. TRACK 2 — The Website `omir.io` (priority 2)

Host on **Cloudflare** (Pages for static, Workers for the playground). **Full build spec: [`WEBSITE.md`](WEBSITE.md).** Information architecture:

```
omir.io/
├─ /            hero: "The open format for agent memory." + diagram + Spec/Try-it/Adopters
├─ /spec        the mdBook spec, versioned (/spec/R1/…)
├─ /playground  paste a .omir file → live validate (compile omir-validate to WASM) + graph viz
├─ /implementations   who speaks OMIR (Veld first; table grows)
├─ /guides      implementer how-tos; migration from Mem0/Letta
├─ /governance  working group, process, ballot, license, roadmap
├─ /maturity    live OMM dashboard per resource
└─ /blog        announcements, ballot results
```

Voice: precise standards-body register (cf. `hl7.org/fhir`, `onnx.ai`, `opentelemetry.io`) — confident, not hypey, honest about maturity. The `/playground` is the conversion moment: it should reuse `omir-validate` (compiled to WASM) and a graph render of the bundle's entities/relationships.

**Sequencing:** ship `/spec` (the static mdBook) first; **`/playground` is blocked on Track 1** — it compiles `omir-validate` to WASM, which must exist before the playground can validate anything.

---

## 7. BACKLOG — Email for omir.io (DECIDED: Cloudflare-native; DO NOT BUILD YET)

Recorded so it's ready when prioritized. **Mail intentionally lags the standard and website.** The 2026 specifics below were fact-checked against official sources this session — ignore any pre-2024 tutorial (see the MailChannels note).

**Decision:** run email **entirely on Cloudflare.** (Chosen over an earlier self-host plan — Mailcow + Resend on a local box — after research surfaced Cloudflare's first-party **Email Service** and the `cloudflare/agentic-inbox` reference. Nothing is self-hosted; nothing is exposed; no local mail server.)

**Architecture (Cloudflare-native):**
- **Inbound:** Cloudflare **Email Routing** = the public MX for `omir.io` — **free, unlimited**; auto-provisions 3 MX (`amir`/`linda`/`isaac.mx.cloudflare.net`) + SPF + DKIM/SRS/ARC. Routes each address (`hello@`, `conduct@`, `working-group@`, catch-all) to an **Email Worker**.
- **Processing (the LLM inbox):** Email Worker → parse MIME with **`postal-mime`** → triage/summarize via **Workers AI** or `fetch()` to the **Anthropic API** for human-facing drafts → persist to **D1** (metadata/labels/tasks), **Vectorize** (semantic search over the inbox; embed with `@cf/baai/bge-base-en-v1.5`), **R2** (raw `.eml`/attachments) → **emit an OMIR memory record** per meaningful message (dogfooding the standard — the inbox becomes a memory source).
- **Outbound:** Cloudflare **Email Service** (public beta, Apr 2026) — sends to **any** recipient via Worker binding/REST/SMTP, **auto-configures SPF/DKIM/DMARC**. **3,000 emails/mo free, then $0.35/1k; requires Workers Paid (~$5/mo).** Threaded auto-reply via `message.reply()` — constraints: incoming must pass DMARC; reply once per event; recipient must equal original sender; sender domain must equal received domain; chains with >100 `References` are dropped.
- **Reference implementation to clone:** **[`cloudflare/agentic-inbox`](https://github.com/cloudflare/agentic-inbox)** — Cloudflare's paved-path "email for agents" app (Email Routing in → postal-mime → Workers AI → R2/Vectorize/Durable Objects → reply/send out, with an Agents SDK + MCP server). **Start here.**

**Human reading of mail:** Cloudflare has **no mailbox/webmail.** For humans, either (a) Email Routing **forwards** the role addresses to an existing personal inbox (simplest, free), or (b) bolt on a cheap mailbox provider later if a dedicated webmail UI is wanted (Purelymail $10/yr · Migadu $19/yr · Fastmail ~$5/mo if OAuth2/JMAP for the agent matters). The LLM inbox handles the *agentic* side regardless.

**OAuth (three meanings):** (A) **identity** ("Sign in with @omir.io") → needs Google Workspace / MS 365 (Cloudflare can't); (B) **gating a UI** (e.g. an admin view of the LLM inbox) → **Cloudflare Access / Zero Trust** (generic OIDC/SAML, Google, GitHub, or email OTP; free tier ≈ up to 50 users — *verify current seat count*); (C) **programmatic** mailbox access → OAuth2 is now effectively mandatory (Gmail API / MS Graph; Microsoft Basic-Auth + EWS retiring across 2026–2027) — only relevant if you add a Google/MS mailbox under (b).

**Don't:** build on MailChannels — its free Cloudflare-Workers email **died 2024-08-31**. Cloudflare Email Service is the native replacement.

**Sources:** developers.cloudflare.com/email-routing/ · blog.cloudflare.com/email-for-agents/ · developers.cloudflare.com/email-service/platform/pricing/ · github.com/cloudflare/agentic-inbox · postal-mime.postalsys.com/docs/guides/cloudflare-workers/ · developers.cloudflare.com/cloudflare-one/integrations/identity-providers/

---

## 8. Conventions & gotchas (learned the hard way)

- **`schemas/` is the single source of truth.** Spec pages are a *projection* of the schemas. Never let prose contradict a schema. Generated pages (`spec/src/resources/*.md`) must not be hand-edited once `omir-gen` exists.
- **Filenames are lowercase.** mdBook builds on **case-sensitive** Linux CI; a link to `MemoryRecord.md` when the file is `memoryrecord.md` works on Windows but **breaks CI**. We already hit and fixed this. Keep a link-checker in CI.
- **Never overclaim maturity.** If a resource is new, it's OMM-0/1. Honesty is a feature of the standard.
- **Keep OMIR vendor-neutral.** Veld is the reference implementation, not the owner. Implementation-specific data goes in `extension[]` under a non-`omir.io` URL.
- **Licensing split is load-bearing** — spec CC-BY-4.0, code Apache-2.0. Don't accidentally license spec text as Apache or vice versa.
- **Don't commit secrets** (Cloudflare API tokens, any mail-provider keys) — they belong in gitignored `.env`, never inline or in commits.

---

## 9. First moves for the incoming agent

1. Open `c:\Repositories\Portll\omir-standard`. Read `README.md`, then `spec/src/overview.md`, then the schemas.
2. Validate the repo: confirm all JSON parses and no `omir.org` remains.
3. `git init` + first commit + push to the `omir-wg` org (see §2); add CI (schema validation, mdBook build, link check).
4. Start **Track 1**: scaffold `omir-validate` against `examples/minimal-bundle.omir`.
5. In parallel, stand up the `omir.io` skeleton on Cloudflare Pages serving the mdBook build (static `/spec` only; `/playground` is blocked on Track 1). See `WEBSITE.md`.
6. Leave **email (§7) untouched** until tracks 1 & 2 are substantially shipped.

**Ask the human before:** registering the GitHub org/repo publicly, publishing anything to `omir.io`, or any outward-facing/irreversible step. Confirm direction; approval in one area doesn't extend to the next.
