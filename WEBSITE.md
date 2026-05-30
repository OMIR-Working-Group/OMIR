# OMIR Website — omir.io (Track 2 design doc)

**Status: DESIGN DOC — not yet built.** No site scaffolding exists in the repo yet.
This specifies what to build for **Track 2 (priority 2, after the Standard)** so a cold
agent can start without guessing. License: **site content CC-BY-4.0; site code Apache-2.0**.

---

## Goal

`omir.io` is the canonical home of the OMIR standard — where implementers **read the
spec**, **validate a `.omir` file**, and **find who else speaks OMIR**. Register: precise
standards-body voice (cf. `hl7.org/fhir`, `onnx.ai`, `opentelemetry.io`). Confident, not
hypey, **honest about maturity** (OMM levels visible). The single most strategic page is
`/implementations` — the project's existential risk is "no second implementer," so the
adopter list is the scoreboard.

## Hosting

- **Cloudflare Pages** for the static site (`omir.io` DNS is already on Cloudflare).
- **Cloudflare Workers** only for `/playground` (later — see Sequencing).
- Custom domain: `omir.io` apex + `www` → Pages.

## Repo layout (decision)

Keep the site **in this repo under `site/`** (monorepo with the spec) for now — the spec
build feeds the site, so co-location avoids a cross-repo build. Split into a separate
`omir-wg/omir.io` repo only if the marketing site later outgrows the spec.

```
site/
├─ public/                # Pages deploy output (built artifact; gitignored)
├─ src/                   # landing pages (plain static HTML first; Astro only if needed)
├─ wrangler.toml          # (or Pages dashboard config)
└─ build.(sh|ts)          # mdbook build spec/ → site/public/spec/R1 ; then copy landing
```

## Build pipeline

1. `mdbook build spec/ -d <abs>/site/public/spec/R1` — **pin mdBook 0.4.x** (same version CI installs; see HANDOFF §5 Toolchain).
2. Build/copy landing pages into `site/public/`.
3. Cloudflare Pages deploys `site/public/`.

Landing framework: **start with plain static HTML.** The spec itself is mdBook; don't
over-tool the shell. Reach for Astro only if component reuse actually demands it.

## Information architecture

```
omir.io/
├─ /              hero: "The open format for agent memory." + diagram + Spec / Try-it / Adopters
├─ /spec          the mdBook spec, versioned (/spec/R1/…)
├─ /playground    paste a .omir file → live validate + graph viz   (BLOCKED on Track 1)
├─ /implementations  who speaks OMIR (Veld first; the scoreboard)
├─ /guides        implementer how-tos; migration from Mem0/Letta
├─ /governance    rendered from GOVERNANCE.md
├─ /maturity      OMM dashboard per resource (generated from schemas/ meta)
└─ /blog          announcements, ballot results
```

## `/playground` — BLOCKED on Track 1

The conversion surface: paste a `.omir` Bundle → **validate** + **graph viz**.
- Compile `omir-validate` (Track 1, Rust) to **WASM** via `wasm-bindgen`/`wasm-pack`; run
  it **client-side** — no server round-trip, nothing leaves the browser (a real privacy
  selling point for a memory standard).
- Render entities/relationships as a force-directed graph (entities = nodes, relationships
  = edges, `salience` = node size).
- **Cannot start until `omir-validate` exists.** Until then, ship `/spec` static only.

## Content sources (don't duplicate — render from the source of truth)

- `/spec` ← `mdbook build spec/`.
- `/governance` ← `GOVERNANCE.md`.
- `/implementations` ← a small committed data file (Veld first), grows as adopters land.
- `/maturity` ← generated from `schemas/` `meta` + spec OMM levels.

## Open decisions (ask the human)

- Monorepo `site/` vs a separate `omir-wg/omir.io` repo.
- Landing framework (plain HTML vs Astro).
- Whether `/playground` ships in v1 or waits for `omir-validate`→WASM (**recommended: wait**).

## Don't

- **Don't publish anything to `omir.io` without human sign-off** (outward-facing, see HANDOFF §9).
- **Don't hand-edit generated `/spec` resource pages** — fix the schema and regenerate (HANDOFF §8).
