#!/usr/bin/env bash
set -euo pipefail

# Bootstrap dev environment for BlockSwap

if ! command -v rustc >/dev/null; then
  echo "Installing Rust via rustup..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source "$HOME/.cargo/env"
fi

if ! command -v cargo >/dev/null; then
  source "$HOME/.cargo/env"
fi

if ! command -v go >/dev/null; then
  echo "Go not found. Please install Go 1.22+ via your package manager."
fi

echo "Environment ready. Try building: cargo build --manifest-path node/Cargo.toml"
