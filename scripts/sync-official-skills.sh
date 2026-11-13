#!/usr/bin/env bash
# Descarga/actualiza las skills OFICIALES de terceros dentro de .claude/skills/.
# Solo copia skills con licencia que permite redistribución (MIT / Apache-2.0).
# Uso: bash scripts/sync-official-skills.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/.claude/skills"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

fetch() { # repo ref subdir skill-names...
  local repo="$1" ref="$2" sub="$3"; shift 3
  local dir="$TMP/${repo//\//_}"
  git clone --quiet --depth 1 --filter=blob:none --sparse ${ref:+--branch "$ref"} "https://github.com/$repo.git" "$dir"
  git -C "$dir" sparse-checkout set "$sub" >/dev/null
  local sha; sha="$(git -C "$dir" rev-parse --short HEAD)"
  for s in "$@"; do
    rm -rf "${DEST:?}/$s"
    cp -R "$dir/$sub/$s" "$DEST/$s"
    printf -- '- `%s` ← https://github.com/%s/tree/%s/%s/%s (commit %s)\n' "$s" "$repo" "${ref:-main}" "$sub" "$s" "$sha" >> "$TMP/sources.md"
    echo "✓ $s ($repo@$sha)"
  done
}

mkdir -p "$DEST"
: > "$TMP/sources.md"

# Supabase — MIT
fetch supabase/agent-skills main skills supabase supabase-postgres-best-practices
# Next.js (Vercel) — MIT. Las skills de referencia de Next.js ahora vienen en
# node_modules/next/dist/docs/ y en el bloque de AGENTS.md que genera `next dev`.
fetch vercel/next.js canary skills next-dev-loop next-bundle-optimizer \
  next-cache-components-adoption next-cache-components-optimizer \
  next-partial-prefetching-adoption next-partial-prefetching-optimizer
# Anthropic — Apache-2.0 (incluye LICENSE.txt dentro de la carpeta)
fetch anthropics/skills main skills frontend-design webapp-testing

# Avisos de licencia MIT a nivel repo (Anthropic los incluye dentro de cada skill)
mkdir -p "$DEST/_licenses"
curl -fsSL https://raw.githubusercontent.com/supabase/agent-skills/main/LICENSE -o "$DEST/_licenses/supabase-agent-skills-MIT.txt"
curl -fsSL https://raw.githubusercontent.com/vercel/next.js/canary/license.md -o "$DEST/_licenses/vercel-nextjs-MIT.txt"

{
  echo "# Skills oficiales de terceros (vendoreadas)"
  echo
  echo "Generado por \`scripts/sync-official-skills.sh\` el $(date -u +%Y-%m-%d). No edites estas carpetas a mano: vuelve a correr el script."
  echo
  cat "$TMP/sources.md"
} > "$DEST/OFFICIAL_SKILLS.md"

echo
echo "Listo. Skills de vercel-labs/agent-skills (sin licencia declarada, no se redistribuyen) — instálalas localmente con:"
echo "  npx skills add vercel-labs/agent-skills --skill deploy-to-vercel --skill react-best-practices --skill web-design-guidelines"
