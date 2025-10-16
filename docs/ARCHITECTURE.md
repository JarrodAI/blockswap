# Architecture

## Core Parameters

- **Consensus Mechanism:** Proof-of-Stake (PoS) with BFT + Block Swap Extension
- **Block Time:** 2 seconds
- **Finality:** 6 seconds (3 blocks)
- **Block Size:** Dynamic, 2MB (min) to 10MB (max), adjusts based on network conditions
- **Block Size Adjustment:** Algorithm increases/decreases size based on recent block fullness and propagation time
- **TPS Target:** 10,000+ transactions per second
- **Validator Set:** 100-200 active validators (rotating)
- **Minimum Stake:** 10,000 BSWP tokens
- **Validator Rewards Formula:**
  - Base reward: 10 BSWP/block
  - Quality bonus: up to 5 BSWP for high-quality blocks
  - Swap bonus: 2 BSWP for successful swap
  - Fee distribution: 80% proposer, 15% voters, 5% burned
  - Penalty: -20% of base reward for low-quality blocks
- **Epoch Length:** 3,600 blocks (~2 hours)
- **Validator Rotation:** At each epoch, rotate validator set using stake-weighted random selection

## Diagrams

- ![System Architecture Diagram](diagrams/system_architecture.png) (source: diagrams/system_architecture.mmd)
- ![Network Topology Diagram](diagrams/network_topology.png) (source: diagrams/network_topology.mmd)
- ![Data Flow Diagram](diagrams/data_flow.png) (source: diagrams/data_flow.mmd)
- ![Consensus Flow Diagram](diagrams/consensus_flow.png) (source: diagrams/consensus_flow.mmd)

## Node Components Overview
- Data Model (Blocks, Tx, State)
- Storage Layer (RocksDB)
- Networking (libp2p)
- Consensus Layer (BFT + Swap)
- Cross-Chain Layer
- VM and Contracts
- RPC and API

## Network Topology

### Node Types
- Validator Node: staked (≥10,000 BSWP), participates in consensus, full state; target uptime ≥99%; HW: 8+ vCPU, 32GB RAM, 1TB SSD, 100Mbps
- Full Node: full history/state, validates and serves RPC; HW: 4+ vCPU, 16GB RAM, 500GB SSD, 50Mbps
- Light Client: headers + proofs only; HW: 2 vCPU, 2GB RAM, 10GB storage
- Archive Node: full historical state (no pruning); HW: 16+ vCPU, 64GB RAM, 5TB SSD

### Communication
- P2P: libp2p (gossipsub for blocks/tx, Kademlia DHT for discovery, noise for transport security)
- Ports: P2P 30333/TCP, RPC 9933 (HTTP), WS 9944, Metrics 9615
- Peer limits: 50 inbound, 25 outbound
- Sync: header-first, fast sync (snapshots), full sync, snap style (optional)

## State Management

### Account Model
- Account-based (Ethereum-like)
- Account: address (32 bytes), balance (uint256), nonce (u64), code hash (32 bytes), storage root (32 bytes), staking data (optional)
- Address derivation: blake2b(pubkey)[0..32]; bech32 prefix: bswp

### State Tree
- Merkle Patricia Trie (Ethereum-compatible)
- Node types: leaf, extension, branch
- Root: 32-byte blake2b-256 included in block header
- Proofs for light clients supported

### Storage Strategy (RocksDB)
- Column families: State, Storage, Code, TrieNodes
- Efficient key encoding, batched writes per block, crash-safe

### State Transitions
- STF applies ordered tx list: verify signature → nonce → balance → execute → events → gas/fees
- On failure: revert state changes; charge gas used

### Pruning & Snapshots
- Prune to last 256 states (configurable); archive nodes do not prune
- Snapshots every 10,000 blocks for fast sync; documented snapshot format

## Gas Mechanism

### Gas Model
- Measures computational/storage cost
- Costs (examples):
  - Transfer: 21,000 gas
  - Storage write/byte: 625 gas; read/byte: 200 gas
  - Crypto ops: 3,000–50,000 gas
  - Contract call: 25,000 + dynamic
  - Contract creation: 32,000 + dynamic
- Limits: per tx 10,000,000 gas; per block 30,000,000 gas
- Pricing: base fee (EIP-1559 style) + priority tip; min 0.0001 BSWP/gas
- Fee: total_fee = gas_used * (base_fee + tip); base fee is burned; tip → proposer

## Finality and Fork Choice

### Finality
- Soft: included (1 conf)
- Economic: ≥2/3 validators vote (≈1 block)
- Absolute: 3 blocks (~6s); checkpoints every 100 blocks (irreversible)

### Fork Choice
- GHOST-like: follow most validator-weighted chain
- Resolution: compare stake-weighted votes; if unclear, wait
- Reorg protection: max depth 3; beyond 3, checkpoints prevent reorg

## Genesis and Bootstrap

### Genesis Block
- Height: 0; Parent: 0x00..00; Timestamp: launch time; State root: initial; Validator set: initial validators
- Initial state: allocations (foundation, sale), validator stakes, system contracts
- Params: block time, gas limits, fees, epoch length
- Format: genesis.json with accounts, validators, params

### Bootstrap Process
- Initial validators coordinate launch; publish bootstrap nodes (seed peers)
- Define network ID and chain ID; joining nodes fetch genesis and peers
- Relayers/infra prepared for anchoring and monitoring
