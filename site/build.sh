#!/usr/bin/env sh
# Build the OMIR site.
#
# Renders the mdBook specification into site/src/spec/R1 — *next to* the landing
# pages — so site/src/ is directly browsable: open site/src/index.html and the
# ./spec/R1/... links resolve with no assembly step. It then assembles the full
# deploy artifact in site/public/ (a copy of site/src/), which is what Cloudflare
# Pages deploys (see WEBSITE.md). Both site/src/spec/ and site/public/ are
# gitignored build output.
#
# Usage:
#   sh site/build.sh
#
# Runs under any POSIX shell, including Git Bash on Windows. Requires mdBook on
# PATH (CI pins a single mdBook version; this repo is built against 0.5.x).
set -eu

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC="$SCRIPT_DIR/src"
SPEC_OUT="$SRC/spec/R1"
OUT="$SCRIPT_DIR/public"

echo "==> Rendering spec: mdbook build -> $SPEC_OUT"
mdbook build "$REPO_ROOT/spec" -d "$SPEC_OUT"

echo "==> Cleaning $OUT"
# Empty the directory's contents rather than removing the directory node, so the
# build still works when a shell or watcher holds $OUT as its working directory
# (common on Windows, where removing a busy directory fails).
mkdir -p "$OUT"
find "$OUT" -mindepth 1 -delete 2>/dev/null || true

echo "==> Assembling deploy artifact -> $OUT"
cp -R "$SRC"/. "$OUT"/

echo "==> Done."
echo "    Browse locally: $SRC/index.html"
echo "    Deploy artifact: $OUT"
