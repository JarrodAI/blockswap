# Deployment Guide

This guide covers deploying BlockSwap nodes for devnet/testnet/mainnet.

## Node Types
- Validator: participates in consensus; requires stake
- Full node: validates and serves RPC
- Archive: full history

## Ports
- P2P: 30333/TCP
- RPC HTTP: 9933
- WebSocket: 9944
- Metrics: 9615

## Quickstart (Devnet)
- Build node (Rust) and use docker-compose (provided) to spin up a local cluster.

## Testnet
- Use `scripts/deploy-validator.sh` with generated configs
- Secure keys; monitor with Prometheus/Grafana

## Mainnet
- Hardened configs, HSM for keys
- Configure backups and alerting

## Kubernetes (TBD)
- Deployment/Service, PVC for data

## Terraform (TBD)
- Provision cloud resources, networking, security groups
