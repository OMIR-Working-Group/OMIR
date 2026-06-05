#!/usr/bin/env sh
# Build the OMIR site for GitHub Pages.
#
# GitHub Pages is configured as "Deploy from a branch" → main → /docs, which
# serves COMMITTED files (GitHub runs no build step). So this script:
#   1. renders the mdBook spec into site/src/spec/R1 — keeping site/src/ directly
#      browsable (open site/src/index.html; ./spec/R1/… links resolve as-is);
#   2. assembles the full site into /docs at the repo root — the published folder,
#      which IS committed (unlike the gitignored intermediates);
#   3. writes /docs/.nojekyll so GitHub serves mdBook's output verbatim (Jekyll
#      otherwise drops files/dirs it doesn't understand).
# A /docs/CNAME (custom domain), if present, is preserved across rebuilds.
#
# Usage: sh site/build.sh   (POSIX shell; Git Bash on Windows is fine). Requires
# mdBook on PATH (built against 0.5.x).
set -eu

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC="$SCRIPT_DIR/src"
SPEC_OUT="$SRC/spec/R1"
OUT="$REPO_ROOT/docs"

echo "==> Rendering spec: mdbook build -> $SPEC_OUT"
mdbook build "$REPO_ROOT/spec" -d "$SPEC_OUT"

echo "==> Cleaning $OUT (preserving CNAME)"
mkdir -p "$OUT"
# Empty the contents (not the dir node, which a shell/watcher may hold on Windows),
# but keep a custom-domain CNAME if the maintainer added one.
find "$OUT" -mindepth 1 ! -name CNAME -delete 2>/dev/null || true

echo "==> Assembling GitHub Pages site -> $OUT"
cp -R "$SRC"/. "$OUT"/
# Disable Jekyll so mdBook assets (e.g. FontAwesome/, files Jekyll would skip) ship as-is.
: > "$OUT/.nojekyll"

echo "==> Done."
echo "    Browse locally:        $SRC/index.html"
echo "    Committed for Pages:   $OUT  (commit this folder; Pages serves main:/docs)"
