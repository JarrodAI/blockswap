# Atomic Swaps

## Introduction
BlockSwap implements protocol-level atomic swaps using Hashed Time-Locked Contracts (HTLCs), a native order book, and an AMM. This enables trustless swaps both within BlockSwap and across external chains (BTC, ETH, etc.) without smart contracts or wrapped tokens.

## HTLC Implementation
- Structure: hash lock (SHA-256), time lock (unix timestamp), sender, recipient, amount, token/asset ID
- Lifecycle: creation, claim (secret reveal), refund (timeout)
- Secret: 32 bytes random, hash: SHA-256
- Time lock: min 1h, max 24h

## Swap Transaction Format
- Type: "AtomicSwap"
- Fields: initiator, counterparty, offered amount/asset, requested amount/asset, hash lock, time lock, chain ID (if cross-chain), nonce, signature
- Validation: correct format, valid hash/time lock, sufficient balance, signature
- Gas costs: per operation

## Swap Matching Engine
- Order book: order ID, maker, offer, request, status, timestamps
- Matching: price ±2% slippage, asset pair, atomic execution
- Partial fills: allowed for >100 BSWP, split HTLC
- Cancellation/expiration: 24h if not filled

## Native AMM
- Formula: constant product (x*y=k)
- Pool: pool ID, reserves, liquidity shares, fee %, creation time
- Add/remove liquidity: mint/burn shares, update reserves
- Swap: input, min output, fee (0.3%), slippage check, update reserves
- Price impact, fee distribution (30% to LPs)
- Liquidity share token: fungible, transferable

## Cross-Chain Atomic Swap Flow
- BTC ↔ BSWP: user creates HTLC on BTC, submits proof to BlockSwap, BlockSwap creates matching HTLC, secret reveal, claim/refund logic
- ETH ↔ BSWP: similar flow
- Proof requirements, timeout/refund, edge cases (network, partial reveals, race)

## Fee Distribution
- Order book: 0.1% fee
- AMM: 0.3% fee
- Distribution: 50% validators, 30% LPs (AMM), 20% burned
- Collected/distributed per block

## Diagrams
- ![HTLC Lifecycle](../diagrams/htlc_lifecycle.png) (source: diagrams/htlc_lifecycle.mmd)
- ![Same-Chain Swap](../diagrams/same_chain_swap.png) (source: diagrams/same_chain_swap.mmd)
- ![Cross-Chain Swap](../diagrams/cross_chain_swap.png) (source: diagrams/cross_chain_swap.mmd)
- ![AMM Mechanics](../diagrams/amm_mechanics.png) (source: diagrams/amm_mechanics.mmd)
- ![Swap Matching Engine](../diagrams/swap_matching_engine.png) (source: diagrams/swap_matching_engine.mmd)
