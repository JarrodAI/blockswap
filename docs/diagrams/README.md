# Diagrams

This folder contains Mermaid source files (.mmd) for all architecture, consensus, cross-chain, and swap diagrams referenced in the docs.

## Exporting to PNG/SVG

Option A: Using Node mermaid-cli (mmdc)
- Install: npm install -g @mermaid-js/mermaid-cli
- Export all: See scripts/export-diagrams.sh

Option B: Using Docker
- If you have Docker, you can run mmdc in a container. Example:
  docker run --rm -v $(pwd):/data minlag/mermaid-cli mmdc -i /data/system_architecture.mmd -o /data/system_architecture.png

Notes:
- Docs reference .png files (e.g., ../diagrams/system_architecture.png). Generate PNGs from the .mmd sources to render them in markdown.
- You can also export SVG by changing the output file extension to .svg.

## CI-based export (recommended)
- A GitHub Actions workflow is provided: `.github/workflows/export-diagrams.yml`.
- It runs automatically on pushes that touch `.mmd` files or the export script, and can also be triggered manually in the Actions tab.
- The workflow renders PNGs and commits them back to the repository.
