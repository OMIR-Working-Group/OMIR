# OMIR Website — omir.io (Track 2 design doc)

**Status: SCAFFOLDED & BUILDING.** A static `site/` now exists: landing pages in
`site/src/`, a `site/build.sh` that renders the mdBook spec into `site/public/spec/R1`
and assembles the pages alongside it, and `site/public/` as the (gitignored) deploy
artifact. Pages live: Home, Spec, Guides, Implementations, Feedback, About. `/playground`
remains **blocked on Track 1** (needs `omir-validate` → WASM; the validator is now
scaffolded). This doc remains the Track 2 contract. License: **site content CC-BY-4.0;
site code Apache-2.0**.

---

## Goal

`omir.io` is the canonical home of the OMIR standard — where implementers **read the
spec**, **validate a `.omir` file**, and **find who else speaks OMIR**. Register: precise
standards-body voice (cf. `hl7.org/fhir`, `onnx.ai`, `opentelemetry.io`). Confident, not
hypey, **honest about maturity** (OMM levels visible). The single most strategic page is
`/implementations` — the project's existential risk is "no second implementer," so the
adopter list is the scoreboard.

## Hosting

- **GitHub Pages** is the current host: **Settings → Pages → Deploy from a branch → `main` → `/docs`.**
  Pages serves the committed `/docs` folder verbatim (a `/docs/.nojekyll` marker disables Jekyll so
  mdBook's output ships intact). Project URL: `https://omir-working-group.github.io/OMIR/`.
- **Custom domain (optional):** add `/docs/CNAME` containing `omir.io` and point DNS at GitHub Pages;
  `site/build.sh` preserves an existing `/docs/CNAME` across rebuilds. (`omir.io` DNS is on Cloudflare,
  so this can be a CNAME/ALIAS to the Pages host, or Cloudflare Pages can serve the same `/docs` tree.)
- **Cloudflare Workers** only for `/playground` (later — see Sequencing).

## Repo layout (decision)

Keep the site **in this repo under `site/`** (monorepo with the spec) for now — the spec
build feeds the site, so co-location avoids a cross-repo build. Split into a separate
`OMIR-Working-Group/omir.io` repo only if the marketing site later outgrows the spec.

```
site/
├─ src/                   # landing pages (plain static HTML first; Astro only if needed)
├─ src/index.html         # friendly OMIR landing page prototype
├─ src/spec/              # mdBook render target (gitignored intermediate)
└─ build.sh               # mdbook build spec/ → site/src/spec/R1 ; then assemble /docs
                          # (committed GitHub Pages root)
```

(`/docs` lives at the **repo root**, not under `site/`, because GitHub Pages publishes
`main:/docs`.)

## Build pipeline

1. `mdbook build spec/ -d <abs>/site/src/spec/R1` — **pin mdBook 0.5.x (0.5.3)** (same version CI installs; see HANDOFF §5 Toolchain). Rendering the spec *next to* the landing pages keeps `site/src/` directly browsable (open `site/src/index.html`; the `./spec/R1/…` links resolve with no assembly step).
2. Assemble the published site: copy `site/src/` → `/docs` (repo root) and write `/docs/.nojekyll`.
3. **Commit `/docs`.** GitHub Pages serves `main:/docs` (no build step on GitHub's side), so the
   assembled output must be committed. A `/docs/CNAME` is preserved across rebuilds.

All steps are wrapped by `site/build.sh` (run `sh site/build.sh`). `site/src/spec/` is a gitignored
intermediate; `/docs` is committed. All site links are **relative**, so the site works whether served
at a domain root (`omir.io/`) or a project subpath (`…github.io/OMIR/`).

Landing framework: **start with plain static HTML.** The spec itself is mdBook; don't
over-tool the shell. Reach for Astro only if component reuse actually demands it.

The initial landing page can include interactive sign-in and in-place editing as a prototype for content preview workflows.

## Information architecture

```
omir.io/
├─ /              hero: "The open format for agent memory." + diagram + Spec / Try-it / Adopters
├─ /spec          the mdBook spec, versioned (/spec/R1/…)
├─ /playground    paste a .omir file → live validate + graph viz   (BLOCKED on Track 1)
├─ /implementations  who speaks OMIR (Veld first; the scoreboard)
├─ /guides        implementer how-tos; migration from Mem0/Letta
├─ /feedback      spec tree + feedback tied to any section/OMM level (local capture → prefilled GitHub issue)
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

- Monorepo `site/` vs a separate `OMIR-Working-Group/omir.io` repo.
- Landing framework (plain HTML vs Astro).
- Whether `/playground` ships in v1 or waits for `omir-validate`→WASM (**recommended: wait**).

## Don't

- **Don't publish anything to `omir.io` without human sign-off** (outward-facing, see HANDOFF §9).
- **Don't hand-edit generated `/spec` resource pages** — fix the schema and regenerate (HANDOFF §8).
