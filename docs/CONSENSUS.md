# Block Swap Consensus (BSC)

## Introduction
Block Swap Consensus (BSC) is a novel PoS-based consensus protocol with a three-phase block production cycle, MEV resistance, and protocol-level atomic swaps. BSC improves on traditional PoS by enabling block replacement (swapping) for optimal ordering and fairness, and by natively supporting cross-chain validation.

## Phases

### Phase 1: Proposal Window (0-1s)
- Duration: 1 second
- Multiple validators (selected via VRF-based weighted random) can propose blocks simultaneously
- Transaction selection:
  - Priority by fee (highest first)
  - Time-weighted fairness (FIFO within fee tiers)
  - System txs prioritized
- Max txs/block: dynamic (up to block size/gas limit)
- Gas limit: 30,000,000
- Cross-chain validations included
- Proposal message: includes block, quality metrics, proposer signature

### Phase 2: Swap Window (1-1.5s)
- Duration: 0.5 seconds
- Validators evaluate all proposals
- Block quality score:
  - 30% total fees
  - 40% fairness score
  - 20% cross-chain validations
  - 10% propagation time
- Swap allowed if new block quality > current best + 5%
- Max 3 swaps per slot
- Swap proposal message: block, quality, validator signature
- Replacement rules: only significantly better blocks accepted

### Phase 3: Finalization (1.5-2s)
- Duration: 0.5 seconds
- BFT voting (2/3+ stake required)
- Vote message: block hash, BLS signature
- If threshold not met, move to next slot
- Slashing for double voting, invalid voting, or proposing invalid blocks
- Block commitment and state transition

## MEV Resistance
- Time-weighted tx ordering (FIFO within fee tiers)
- Fairness score penalizes suspicious ordering
- MEV detection heuristics
- Validators penalized for MEV extraction

## Incentives
- Base reward: 10 BSWP/block
- Quality bonus: up to 5 BSWP
- Swap bonus: 2 BSWP
- Fee distribution: 80% proposer, 15% voters, 5% burned
- Penalty: -20% of base reward for low-quality blocks

## Diagrams
- ![Consensus Flow](../diagrams/consensus_flow.png) (source: diagrams/consensus_flow.mmd)
- ![Swap Scenario](../diagrams/swap_scenario.png) (source: diagrams/swap_scenario.mmd)
- ![State Machine](../diagrams/consensus_state_machine.png) (source: diagrams/consensus_state_machine.mmd)
- ![Validator Decision Flow](../diagrams/validator_decision_flow.png) (source: diagrams/validator_decision_flow.mmd)
- ![2s Timing Cycle](../diagrams/timing_2s_cycle.png) (source: diagrams/timing_2s_cycle.mmd)

## BFT Phases and Aggregated Signatures (BLS)
- Tendermint-style pre-vote, pre-commit, commit
- BLS signature aggregation for efficient voting
