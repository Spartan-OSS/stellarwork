# StellarWork (working name)

An experimental milestone escrow on Stellar/Soroban for clients and remote workers.
Clients fund agreed milestones; workers submit evidence references; clients approve
payment or either party requests an agreed arbitrator's split decision.

## Current status

This is a contributor foundation, not a released payment product. The Python business
model passes 13 tests. The Soroban contract passes all four native tests and builds
to release WASM with locked dependencies (validated 2026-10-09).
No contract has been deployed. No wallet integration or frontend is implemented.
See [readiness](docs/READINESS.md) before assigning work and
[publication handoff](docs/PUBLISHING.md) for repository setup.

## Run now (no third-party Python packages)

```bash
python3 -m unittest discover -s model -v
python3 model/demo.py
```

Expected demo: milestone states `Completed` and `Resolved`; client balance 840,
freelancer balance 160, locked balance 0 (abstract integer token units).

## Validate the Soroban baseline

The repository pins Rust 1.99.0 in rust-toolchain.toml, including rustfmt and the
wasm32v1-none target. Install rustup using the linked Stellar setup guide.
The SDK is deliberately pinned to 23.0.0 as an initial baseline, not claimed as latest.
The resolved dependency baseline is recorded in Cargo.lock.

```bash
rustup target add wasm32v1-none
cargo test --workspace --locked
cargo build --target wasm32v1-none --release --locked -p stellarwork-escrow
cargo fmt --all -- --check
```

Cargo.lock is included; preserve it when reproducing this baseline. Required jobs
are `model` and `contract`. CI uses --locked and checks Rust formatting.
A workflow file alone does not configure branch protection.

## Map

| Path | Purpose |
| --- | --- |
| contracts/escrow | Soroban contract and native test source |
| model | Independently executable business specification and demo |
| docs/SPEC.md | API, authorization, states, invariants, architecture |
| docs/DECISIONS.md | Design choices and maintainer decisions |
| docs/READINESS.md | Release/assignment gates and evidence |
| docs/issues | Copy-ready issue bodies with scope and prerequisites |
| CONTRIBUTING.md | Local workflow and review expectations |

Start with [the issue index](docs/issues/INDEX.md). Draft IDs such as SW-004 are local
backlog identifiers, not existing GitHub issue numbers. Work is private/local until
an organization repository is selected and published. No contributors are assigned.

The intended testnet deployment uses one reviewed Stellar Asset Contract per instance.
Token units are i128 integers; the UI must read asset decimals. Do not label arbitrary
assets USDC. Private deliverables stay off-chain; only evidence references are public.

## References

- [Stellar setup](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- [Stellar token integration](https://developers.stellar.org/docs/build/guides/tokens/stellar-asset-contract)
- [Stellar TTL testing](https://developers.stellar.org/docs/build/guides/archival/test-ttl-extension)
- [Drips maintainer workflow](https://docs.drips.network/wave/maintainers/participating-in-a-wave/)

License decision is maintainer-owned and unresolved. There is no invented copyright
holder or license grant in this bundle. Select an open-source license before inviting
external code submissions or applying to a Wave Program.
