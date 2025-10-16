#!/usr/bin/env bash
set -euo pipefail

# Export all Mermaid .mmd files in docs/diagrams to PNG by default.
# Requires: @mermaid-js/mermaid-cli (mmdc) installed globally or available in PATH.

DIAGRAM_DIR="$(cd "$(dirname "$0")/../docs/diagrams" && pwd)"

if ! command -v mmdc >/dev/null 2>&1; then
  echo "Error: mmdc (Mermaid CLI) not found in PATH. Install with: npm i -g @mermaid-js/mermaid-cli" >&2
  exit 1
fi

cd "$DIAGRAM_DIR"

for f in *.mmd; do
  base="${f%.mmd}"
  echo "Rendering $f -> ${base}.png"
  mmdc -i "$f" -o "${base}.png" -b transparent
  # Uncomment to also render SVG
  # mmdc -i "$f" -o "${base}.svg" -b transparent
done

echo "Done. Outputs saved in $DIAGRAM_DIR"