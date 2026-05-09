# Contributing to Stellar-PayStreams

Thank you for your interest in contributing! This document explains the repository layout, how to pick up work, and the standards we hold contributions to.

## Repository Layout

```
Stellar-PayStreams/
├── contracts/stream/    # Soroban smart contract written in Rust
├── frontend/            # React + Radix UI dashboard
└── tests/integration/   # TypeScript integration tests (Vitest)
```

Each directory is an independent workspace. You only need to set up the toolchain for the area you are working in.

## Areas of Contribution

### `contracts/stream` — Smart Contract (Rust / Soroban)
- Requires Rust stable + `wasm32-unknown-unknown` target
- Run tests: `cargo test`
- Labels: `contract`, `rust`

### `frontend` — Dashboard (React / TypeScript / Radix UI)
- Requires Node.js ≥ 18
- Run dev server: `npm run dev`
- Run tests: `npm test`
- Labels: `frontend`, `typescript`

### `tests/integration` — Integration Tests (TypeScript / Vitest)
- Requires Node.js ≥ 18
- Run tests: `npm test`
- Labels: `testing`, `typescript`

## Workflow

1. **Find an issue** — Browse [open issues](https://github.com/Stellar-Project-Hub/Stellar-PayStreams/issues). Issues tagged `good first issue` are beginner-friendly.
2. **Comment to claim** — Leave a comment on the issue saying you'd like to work on it. A maintainer will assign it to you.
3. **Fork & branch** — Fork the repo and create a branch named `<type>/<short-description>` (e.g., `feat/stream-creation-contract`).
4. **Implement** — Follow the implementation steps in the issue body. Meet the Definition of Done before opening a PR.
5. **Open a PR** — Target the `main` branch. Reference the issue with `Closes #<number>`.
6. **CI must pass** — All `cargo test` and `npm test` checks must be green.
7. **Review** — A maintainer will review within 5 business days.

## Code Standards

- **Rust**: `cargo fmt` and `cargo clippy -- -D warnings` must pass.
- **TypeScript**: ESLint + Prettier. Run `npm run lint` before pushing.
- **Commits**: Use [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `chore:`, `test:`, `docs:`).
- **Tests**: Every contract change must include or update unit tests. Every frontend feature must include a component test.

## Issue Labels

| Label | Meaning |
|---|---|
| `contract` | Soroban / Rust smart contract work |
| `frontend` | React dashboard work |
| `testing` | Integration or unit test work |
| `enhancement` | New feature or improvement |
| `bug` | Something is broken |
| `good first issue` | Suitable for first-time contributors |
| `help wanted` | Maintainers welcome external help |

## Code of Conduct

Be respectful and constructive. We follow the [Contributor Covenant](https://www.contributor-covenant.org/).

## Questions?

Open a [Discussion](https://github.com/Stellar-Project-Hub/Stellar-PayStreams/discussions) or ask in the issue thread.
