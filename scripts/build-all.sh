#!/usr/bin/env bash
set -euo pipefail

echo "Building Rust node"
( cd node && cargo build )

echo "Building Go CLI"
( cd cli && go build ./... )

echo "Done."
