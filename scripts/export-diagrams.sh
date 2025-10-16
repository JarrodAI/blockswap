#!/usr/bin/env bash
set -euo pipefail

# Export all Mermaid .mmd files in docs/diagrams to PNG by default.
# Requires: @mermaid-js/mermaid-cli (mmdc) installed globally or available in PATH.

DIAGRAM_DIR="$(cd "$(dirname "$0")/../docs/diagrams" && pwd)"

if ! command -v mmdc >/dev/null 2>&1; then
  echo "Error: mmdc (Mermaid CLI) not found in PATH. Install with: npm i -g @mermaid-js/mermaid-cli" >&2
  exit 1
fi

echo "Using mmdc at: $(command -v mmdc)"
echo "mmdc version: $(mmdc --version || echo unknown)"

cd "$DIAGRAM_DIR"

shopt -s nullglob
files=( *.mmd )
if [ ${#files[@]} -eq 0 ]; then
  echo "No .mmd files found in $DIAGRAM_DIR"
  exit 0
fi

echo "Found ${#files[@]} .mmd files: ${files[*]}"

for f in "${files[@]}"; do
  base="${f%.mmd}"
  echo "Rendering $f -> ${base}.png"
  mmdc -i "$f" -o "${base}.png" -b transparent || {
    echo "Failed to render $f" >&2
    exit 1
  }
  # Uncomment to also render SVG
  # mmdc -i "$f" -o "${base}.svg" -b transparent
done

echo "Done. Outputs saved in $DIAGRAM_DIR"