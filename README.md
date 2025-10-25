# BlockSwap

BlockSwap is a novel Layer-1 blockchain featuring native cross-chain block validation, protocol-level atomic swaps, and a MEV-resistant Block Swap Consensus (BSC).

This repository contains the monorepo for the project: the Rust node, Go CLI tools, SDK stubs, docs, and deployment scripts.

Quickstart (dev)

- Build the node: see `node/`.
- Build the CLI: see `cli/`.
- Spin up a local multi-node testnet (WIP): see `docker-compose.yml`.

Repository layout (early scaffold)

- `node/` — Rust core node (binary)
- `cli/` — Go CLI tools
- `docs/` — Whitepaper and technical documentation
- `scripts/` — Helper scripts (genesis, deploy, monitor)
- `deployment/` — Infra scaffolding (Kubernetes/Terraform placeholders)
- `tests/` — Integration/e2e/perf test placeholders

Status: initial scaffolding. See `master.md` for the full blueprint.