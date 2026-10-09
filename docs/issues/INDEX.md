# Contributor backlog

Ten copy-ready drafts. Most are deliberately blocked until the contract/wallet baseline
is validated. Complexity estimates are recommendations; they are not issued rewards.
Do not assign all ten at once. Initial readiness work stays with the maintainer.

| ID | Outcome | Readiness | Complexity | Dependencies |
| --- | --- | --- | --- | --- |
| [SW-001](sw-001.md) | Validate and lock the native Soroban baseline | Maintainer-owned | High | G0; no code prerequisites |
| [SW-002](sw-002.md) | Prove role authorization with explicit Soroban auth trees | Blocked | High | SW-001 |
| [SW-003](sw-003.md) | Verify atomic rollback and isolation for SAC transfer failures | Blocked | High | SW-001 |
| [SW-004](sw-004.md) | Test persistent-entry TTL extension and keep_alive boundaries | Blocked | Medium | SW-001 |
| [SW-005](sw-005.md) | Add machine-readable lifecycle fixtures for model and future SDK tests | Ready after G0 | Medium | G0 only; runnable model exists |
| [SW-006](sw-006.md) | Prove three-party wallet authorization on testnet in a small harness | Blocked | High | SW-001, SW-002, SW-003, SW-004; G3 deployment metadata |
| [SW-007](sw-007.md) | Build read-only engagement and milestone pages | Blocked | Medium | SW-005, SW-006; G4 |
| [SW-008](sw-008.md) | Add the client fund and approve transaction flows | Blocked | High | SW-006, SW-007 |
| [SW-009](sw-009.md) | Implement worker submission and party dispute controls | Blocked | Medium | SW-007, SW-008 |
| [SW-010](sw-010.md) | Add arbitrator split settlement with exact amount validation | Blocked | High | SW-003, SW-007, SW-008 |

## Assignment batches

1. Maintainer completes G0 and SW-001. SW-005 may then run alongside contract verification.
2. SW-002, SW-003, SW-004 can be assigned to separate contributors after SW-001; split
   their test additions into separate modules to reduce conflicts in test.rs.
3. Maintainer deploys testnet and records G3; assign SW-006 as a compatibility proof.
4. Assign SW-007, then SW-008. SW-009 and SW-010 can proceed in parallel only after their
   shared adapter is merged and paths are separated.

## Maintainer work outside reward issues

- Select repository/license; name reviewers; configure required checks.
- Validate demand and select a pilot arbitration/privacy/liveness policy.
- Deploy testnet with a reviewed SAC; record contract ID, network, issuer, decimals,
  WASM checksum, Git commit, tool versions, and three role accounts (public addresses).
- Run approve/dispute/refund smoke flows and store redacted receipts.
- Apply public organization repository to Drips; wait for organizer approval.
- Confirm point budgets and cycle dates, then add only ready issues to the program.
- Review applications quickly and resolve accepted work according to Wave timing rules.

No live GitHub issues, assignments, deployments, or Wave applications were created.
