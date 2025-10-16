# Security Overview

This document summarizes security practices for BlockSwap.

## Node Security
- Input validation, rate limiting, connection limits
- Encrypted keys at rest
- Regular updates, monitoring, alerts

## Consensus Security
- Slashing for double-signing/downtime
- Finality checkpoints and reorg limits

## Smart Contracts
- WASM sandboxing, gas metering
- Avoid dangerous host functions

## Cross-Chain
- Multi-confirmation requirements
- Fraud proofs (future), relayer bonds and reputation

## Audits & Bug Bounty
- Plan multi-stage audits (consensus, cross-chain, crypto)
- Public bug bounty program post-testnet
