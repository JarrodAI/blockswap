# Minimal dev image to build and run the node
FROM rust:1.81 as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blockswap-node /usr/local/bin/blockswap-node
ENV RUST_LOG=info
WORKDIR /data
EXPOSE 30333 9933 9944
ENTRYPOINT ["/usr/local/bin/blockswap-node"]
