# Database Schema (Indexer/backend)

PostgreSQL schema outline for the explorer/indexer backend.

## Tables
- blocks(id serial, height bigint, hash bytea, parent_hash bytea, timestamp timestamptz, tx_count int)
- transactions(id serial, hash bytea, block_height bigint, from_addr bytea, to_addr bytea, amount numeric, fee numeric, nonce bigint, status text)
- accounts(address bytea primary key, balance numeric, nonce bigint)
- validators(address bytea primary key, stake numeric, status text, last_seen timestamptz)
- swaps(id serial, order_id text, maker bytea, taker bytea, offer jsonb, request jsonb, status text, created_at timestamptz)
- anchors(id serial, chain_id text, height bigint, hash bytea, timestamp timestamptz)

Indexes and optimizations to be added as needed.
