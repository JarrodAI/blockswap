#!/usr/bin/env bash
set -euo pipefail

echo "Testing Rust node"
( cd node && cargo test )

echo "Testing Go CLI"
( cd cli && go test ./... )

echo "Done."
