#!/usr/bin/env bash
set -euo pipefail

echo "[genesis] Generating genesis.json (stub)"
mkdir -p .blockswap
cat > .blockswap/genesis.json <<'JSON'
{
  "chain_id": "blockswap-dev",
  "validators": [],
  "alloc": {}
}
JSON

echo "[genesis] Wrote .blockswap/genesis.json"
