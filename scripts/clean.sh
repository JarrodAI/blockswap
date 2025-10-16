#!/usr/bin/env bash
set -euo pipefail

find node/ -name target -type d -prune -exec rm -rf {} + || true
find . -name "*.log" -type f -delete || true
echo "Clean complete."
