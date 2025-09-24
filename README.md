# DeKu

Root project structure for Substrate + decentralized Kubernetes deployment
Project layout:
deku/
├── Cargo.toml (workspace)
├── node/         (Substrate node template)
├── pallets/
│   └── deku/     (Custom pallet for job submission)
├── cli/          (Rust CLI using subxt to submit jobs)
├── agent/        (Rust daemon that watches chain for jobs)
└── types/        (Shared structs and types)

## Setup

Get substrate using the `get_substrate.sh` script.

Then startup a node:

```
substrate-contracts-node --dev
```

Install cargo-contract:

```
cargo install cargo-contract --force
```
