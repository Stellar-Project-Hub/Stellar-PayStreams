# Stellar-PayStreams

A decentralized protocol for creating real-time token streams and recurring subscriptions on [Soroban](https://soroban.stellar.org), complete with a frontend dashboard for managing active streams.

## Overview

Stellar-PayStreams enables continuous, per-second token transfers between accounts on the Stellar network. Use cases include payroll, subscriptions, vesting schedules, and any time-based payment flow.

## Monorepo Structure

```
Stellar-PayStreams/
├── contracts/
│   └── stream/          # Soroban smart contract (Rust)
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
├── frontend/            # React dashboard (TypeScript + Radix UI)
│   ├── src/
│   ├── index.html
│   └── package.json
├── tests/
│   └── integration/     # Integration tests (TypeScript + Vitest)
│       ├── src/
│       └── package.json
├── .github/
│   └── workflows/
│       └── ci.yml
├── README.md
└── CONTRIBUTING.md
```

## Tech Stack

| Layer | Technology |
|---|---|
| Smart Contract | Rust, Soroban SDK |
| Frontend | React, TypeScript, Radix UI |
| Testing | Vitest, TypeScript |
| CI | GitHub Actions |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- Node.js ≥ 18

### Contracts

```bash
cd contracts/stream
cargo build --target wasm32-unknown-unknown --release
cargo test
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Integration Tests

```bash
cd tests/integration
npm install
npm test
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines, issue workflow, and how to claim tasks from the backlog.

## License

Apache-2.0 © [Stellar-Project-Hub](https://github.com/Stellar-Project-Hub)
