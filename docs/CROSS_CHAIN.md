# Cross-Chain Protocol

## Introduction
BlockSwap enables native cross-chain validation by anchoring block headers from external chains (Bitcoin, Ethereum, BSC, Polygon, Avalanche) and verifying events using light clients and Merkle proofs. This design supports both trustless and trust-minimized approaches.

## Block Anchoring Protocol
- Anchoring: storing external block headers in BlockSwap state
- Supported chains: Bitcoin, Ethereum, Binance Smart Chain, Polygon, Avalanche C-Chain
- Submission: approved relayers or validators submit block header + metadata
- Verification: proof-of-work (BTC), proof-of-stake (ETH), finality checks
- Anchoring fee: 0.01 BSWP per anchor
- Anchor transaction format: chain ID, block hash, height, timestamp, parent hash, proof data, submitter signature
- Anchor validation: must be valid, sequential, finalized
- Storage: state tree with efficient lookup (chain ID + height), prune to last 10,000 anchors/chain

## Light Client Design
### Bitcoin
- Block header: 80 bytes
- Verify PoW, 6 confirmations
- SPV proof for tx inclusion (Merkle proof)
- Store anchored headers

### Ethereum
- Block header: post-merge PoS
- Verify finality: 2 epochs (~13 min)
- Merkle Patricia Trie for state/receipts
- Store anchored headers

### Other Chains
- Similar specs for BSC, Polygon, Avalanche
- Chain-specific finality and proof formats

## Cross-Chain Event System
- Event types: token lock, burn, contract call, block finalized
- Event proof: Merkle proof + anchor reference
- Users claim events on BlockSwap

## Relayer Network
- Relayers submit external headers
- Registration: bond 1,000 BSWP
- Reputation: track accuracy, penalize invalid submissions, reward accuracy
- Rewards: 0.005 BSWP/anchor
- Slashing: 10% for invalid, 5% for non-finalized, removal for repeated failures
- Rotation: periodic

## Diagrams
- ![Anchor Submission Flow](../diagrams/anchor_submission.png) (source: diagrams/anchor_submission.mmd)
- ![Cross-Chain Verification](../diagrams/cross_chain_verification.png) (source: diagrams/cross_chain_verification.mmd)
- ![BTC to BlockSwap Sequence](../diagrams/btc_to_blockswap.png) (source: diagrams/btc_to_blockswap.mmd)
- ![ETH to BlockSwap Sequence](../diagrams/eth_to_blockswap.png) (source: diagrams/eth_to_blockswap.mmd)
- ![Light Client Verification](../diagrams/light_client_verification.png) (source: diagrams/light_client_verification.mmd)
