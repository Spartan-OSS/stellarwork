# Readiness and validation evidence

Authoring date: 2026-10-03. Local validation updated: 2026-10-09.
Stage: locally validated contributor foundation; public repository setup pending.

## Evidence obtained

- Python 3.12.14: `python3 -m unittest discover -s model -v` — 13 tests passed.
- Randomized model regression: 250 sequences x 40 attempts = 10,000 action attempts,
  seed 20261003. Conservation checked after every success/failure; failed operations
  restore the complete model snapshot.
- Demo: Completed + Resolved; client 840; worker 160; locked 0.
- Auth roles, exact splits, replay, deadlines, IDs, URI bounds, insufficient balance,
  and cross-engagement isolation have model regression coverage.
- Native Soroban tests: 4 passed, 0 failed, using locked dependencies.
- Release WASM build succeeds with wasm32v1-none and --locked.
- cargo fmt --all -- --check passes.
- rustc 1.99.0 (b940084d7 2026-09-28); cargo 1.99.0 (5f94df478 2026-08-27).
- SDK remains pinned to 23.0.0; Cargo.lock is included.
- Compatible ed25519-dalek 2.2.0 is locked: the initial resolution selected 3.0.0
  through soroban-env-host's >=2.0.0 constraint and failed its testutils RNG bound.
- An empty compiler object interrupted one native rebuild; clearing the stellar-xdr
  build cache and rebuilding with one job and incremental compilation off succeeded.
- Five deprecation warnings remain for Events::publish; no event ABI was changed.
- Validation used the installed stable alias, which resolved to the exact compiler
  above. rust-toolchain.toml and CI pin that version for subsequent runs.
- Artifact: target/wasm32v1-none/release/stellarwork_escrow.wasm; 28,132 bytes.
- A copy is included in artifacts/stellarwork_escrow.wasm with artifacts/SHA256SUMS.
- WASM SHA-256: b266f9730e243cc898b49fc86b90f520aea4a9e25f725bd59730cf162646f467.
- CI configuration is present; no GitHub CI runs or branch protections have occurred.

## Gates

| Gate | Owner | Complete when | Current state |
| --- | --- | --- | --- |
| G0 Public project setup | Goodness/maintainer | repository, LICENSE, named reviewers, CI required checks | pending repository choice |
| G1 Native baseline | maintainer | cargo tests + WASM build green; SDK/toolchain recorded; lockfile committed | local checks passed; GitHub CI/reviewer confirmation pending |
| G2 Contract verification | contract reviewer | SW-002 through SW-004 merged with real auth/rollback/TTL evidence | not started |
| G3 Testnet lifecycle | maintainer | deployment metadata + reproduce approved, disputed, expired flows | not started |
| G4 Wallet compatibility | integration reviewer | SW-006 can collect 3-party authorizations on testnet | not started |
| G5 Contributor feature wave | maintainer | unblocked SW-007 onward published with owner/test fixtures | pending G4 |
| G6 External pilot | maintainers | liveness/privacy decisions, reviewed accounting, end-to-end tests | not ready |

Docs/model contributors can start after G0. Contract test contributors can start after G1.
No wallet/frontend feature is advertised as ready before its prerequisites pass.
Drips participation additionally requires organizer acceptance, a real public org repo,
and compliance with current budgets/program rules. Creating a label is not approval.

## Review acceptance

The assigned maintainer must reproduce the issue's commands and compare results against
acceptance criteria. For financial paths inspect resulting balances, authorization trees,
failed-transfer rollback, and terminal-state replays. Model passing is never a substitute
for native contract tests. No synthetic 'audit complete' or 'production ready' claim.

## Remaining baseline acceptance

SW-001 local tests/build/formatting are complete. G0, actual green GitHub CI checks,
and maintainer review remain pending. Keep SW-002 through SW-004 blocked until those
are confirmed. The four baseline native tests mock authorization and do not prove
missing-signature rejection, failed-transfer rollback, or TTL behavior.
