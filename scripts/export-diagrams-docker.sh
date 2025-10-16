#!/usr/bin/env bash
set -euo pipefail

DIAGRAM_DIR="$(cd "$(dirname "$0")/../docs/diagrams" && pwd)"

echo "Using Dockerized Mermaid CLI"

cd "$DIAGRAM_DIR"

for f in *.mmd; do
  base="${f%.mmd}"
  echo "Rendering $f -> ${base}.png"
  docker run --rm -v "$PWD":/data minlag/mermaid-cli mmdc -i "/data/$f" -o "/data/${base}.png" -b transparent
  # Uncomment to also render SVG
  # docker run --rm -v "$PWD":/data minlag/mermaid-cli mmdc -i "/data/$f" -o "/data/${base}.svg" -b transparent
done

echo "Done. Outputs saved in $DIAGRAM_DIR"
