# Development Guide

This document describes how to build, run, and contribute to BlockSwap.

## Prerequisites
- Rust (stable + nightly), cargo, clippy, rustfmt
- Go 1.21+
- Node.js (for tooling; optional)
- Docker (optional)

## Build
- Node (Rust): `cargo build` in `node/`
- CLI (Go): `go build ./...` in `cli/`

## Run (local devnet)
- Use docker-compose (TBD) or run the Rust node binary directly. A simple two-node compose file is provided.

## Testing
- Rust: `cargo test` in `node/`
- Go: `go test ./...` in `cli/`

## Linting
- Rust: `cargo clippy --all-targets --all-features -D warnings`
- Format: `cargo fmt --all`

## Diagrams
- Generate PNGs from Mermaid sources: `scripts/export-diagrams.sh`

## Contribution Workflow
- Create a branch, open PRs, keep changes small
- Include tests where applicable
- Update docs if behavior changes
- Follow `CODE_OF_CONDUCT.md` and `CONTRIBUTING.md` (TBD)
