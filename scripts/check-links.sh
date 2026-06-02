#!/usr/bin/env bash
# Internal link & anchor checker for the built OMIR site.
#
#   usage: scripts/check-links.sh [SITE_ROOT]      (default: site/src)
#
# Run `sh site/build.sh` first so the spec is rendered under <root>/spec/R1.
# Two checks, both fatal:
#   1. landing-page relative links (href="./page.html") point at a real file;
#   2. intra-spec deep links (href="...#anchor") resolve to an existing
#      element id in the target rendered page.
#
# This is the link-checker CI gate (HANDOFF §5) and is safe to run locally.
set -uo pipefail

ROOT="${1:-site/src}"
fail=0

if [ ! -d "$ROOT" ]; then
  echo "check-links: site root not found: $ROOT (did you run 'sh site/build.sh'?)" >&2
  exit 2
fi

# 1. Landing-page relative links between pages (ignore #fragments and externals).
while IFS= read -r link; do
  [ -z "$link" ] && continue
  if [ ! -f "$ROOT/$link" ]; then
    echo "BROKEN LINK: ./$link (referenced by a landing page)"
    fail=1
  fi
done < <(grep -rhoE 'href="\./[^"#]+"' "$ROOT"/*.html 2>/dev/null | sed -E 's/href="\.\///; s/"$//' | sort -u)

# 2. Intra-spec deep links (#anchors) in the rendered mdBook.
SPEC="$ROOT/spec/R1"
if [ -d "$SPEC" ]; then
  while IFS= read -r f; do
    dir=$(dirname "$f")
    rel=${f#"$SPEC"/}
    while IFS= read -r link; do
      case "$link" in http*|mailto*|"") continue ;; esac
      file=${link%%#*}
      anchor=${link#*#}
      [ -z "$anchor" ] && continue
      if [ -z "$file" ]; then target="$f"; else target="$dir/$file"; fi
      if [ ! -f "$target" ]; then
        echo "[$rel] missing file for link: $link"
        fail=1
        continue
      fi
      if ! grep -q "id=\"$anchor\"" "$target"; then
        echo "[$rel] broken anchor: $link"
        fail=1
      fi
    done < <(grep -oE 'href="[^"]*#[^"]*"' "$f" 2>/dev/null | sed -E 's/href="//; s/"$//')
  done < <(find "$SPEC" -name '*.html')
else
  echo "check-links: no rendered spec at $SPEC — run 'sh site/build.sh' first" >&2
  exit 2
fi

if [ "$fail" -ne 0 ]; then
  echo "Link check FAILED."
  exit 1
fi
echo "Link check passed: all internal links and intra-spec anchors resolve."
