# Contributing

This draft is prepared for a future public repository. Check docs/READINESS.md and the
issue index before starting; blocked issues should not be assigned yet. Reviewer identities
must be completed before accepting external code contributions.

1. Comment on the published issue with an approach, dependencies, and expected delivery.
2. Wait for assignment; one primary contributor per issue avoids duplicated work.
3. Reproduce the baseline commands from README before changing behavior.
4. Stay within the specified paths/scope. Propose API changes before implementation.
5. Include a focused regression for changed financial/auth behavior. Keep dependency
   additions and formatting unrelated to the issue out of the PR.
6. Link the real issue number using Closes #N, complete the PR template, and report exact
   commands/results. Never include wallet keys, credentials, or private client evidence.

Maintainer review targets (proposed): assignment within 48 hours; first PR feedback within
2 working days. Contributors post a progress note every 2–3 days and report blockers
promptly. These are project expectations, not Drips guarantees.

Required GitHub checks once activated: model and contract. Configure a ruleset on main
with PR review, required checks, stale review dismissal, and no casual bypass. Add named
CODEOWNERS only after actual reviewers accept ownership; paths alone do not create expertise.

For Wave issues, verify the current cycle and repository/org points budgets before
publishing. Do not promise a fixed currency payout: complexity points and reward amounts
are different. Track resolved status before the active Wave ends per organizer rules.

## Pre-Pull Request Checklist

Before submitting a Pull Request, please ensure that your changes pass all local formatting, compilation, and testing checks. This saves CI resources and speeds up the review process.

1. **Format Code**: Ensure your code is properly formatted to avoid style-related CI failures.
   ```bash
   cargo fmt --all
   ```

2. **Build and Check for Warnings**: Build the smart contract and ensure there are no compilation or deprecation warnings.
   ```bash
   cargo build --target wasm32v1-none --release --locked -p stellarwork-escrow
   ```

3. **Run Unit Tests**: Run the full test suite locally. All tests must pass before opening a PR.
   ```bash
   cargo test --locked
   ```
