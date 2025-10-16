# BlockSwap Design Summary

- Consensus: PoS + BFT + Block Swap (2s blocks, 6s finality)
- Throughput: 10k+ TPS target; dynamic block size 2–10MB
- Validators: 100–200 active; min stake 10,000 BSWP; epoch 3,600 blocks
- Fees: EIP-1559-style base fee + tip; base burned
- Incentives: base 10 BSWP/block, quality bonus up to 5, swap bonus 2, fees 80/15/5 (proposer/voters/burn)
- Cross-chain: anchoring BTC/ETH/BSC/Polygon/Avalanche; SPV + PoS finality checks; relayer network (bond 1,000 BSWP)
- Atomic swaps: protocol HTLC; order book + AMM (0.3% fee)
- Networking: libp2p gossipsub + Kademlia; encrypted (noise)
- State: account-based; MPT; RocksDB; pruning + snapshots
- Finality/forks: checkpoints every 100 blocks; GHOST-like fork choice; max reorg depth 3

See detailed specs:
- ARCHITECTURE.md
- CONSENSUS.md
- CROSS_CHAIN.md
- ATOMIC_SWAPS.md

## Diagrams (with sources)
- System Architecture: diagrams/system_architecture.png (source: diagrams/system_architecture.mmd)
- Network Topology: diagrams/network_topology.png (source: diagrams/network_topology.mmd)
- Data Flow: diagrams/data_flow.png (source: diagrams/data_flow.mmd)
- Consensus Flow: diagrams/consensus_flow.png (source: diagrams/consensus_flow.mmd)
- Consensus State Machine: diagrams/consensus_state_machine.png (source: diagrams/consensus_state_machine.mmd)
- Swap Scenario: diagrams/swap_scenario.png (source: diagrams/swap_scenario.mmd)
- Anchor Submission: diagrams/anchor_submission.png (source: diagrams/anchor_submission.mmd)
- Cross-Chain Verification: diagrams/cross_chain_verification.png (source: diagrams/cross_chain_verification.mmd)
- BTC → BlockSwap: diagrams/btc_to_blockswap.png (source: diagrams/btc_to_blockswap.mmd)
- ETH → BlockSwap: diagrams/eth_to_blockswap.png (source: diagrams/eth_to_blockswap.mmd)
- Light Client Verification: diagrams/light_client_verification.png (source: diagrams/light_client_verification.mmd)
- HTLC Lifecycle: diagrams/htlc_lifecycle.png (source: diagrams/htlc_lifecycle.mmd)
- Same-Chain Swap: diagrams/same_chain_swap.png (source: diagrams/same_chain_swap.mmd)
- Cross-Chain Swap: diagrams/cross_chain_swap.png (source: diagrams/cross_chain_swap.mmd)
- AMM Mechanics: diagrams/amm_mechanics.png (source: diagrams/amm_mechanics.mmd)
- Swap Matching Engine: diagrams/swap_matching_engine.png (source: diagrams/swap_matching_engine.mmd)

Quick note: PNGs are generated from the Mermaid sources using scripts/export-diagrams.sh.

## Key ports & protocols
- P2P: 30333/TCP (libp2p Gossipsub + Kademlia, Noise encryption)
- RPC (HTTP): 9933
- WebSocket: 9944
- Metrics: 9615
