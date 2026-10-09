# Repository publication handoff

## Decisions needed

- Select the GitHub account or organization and repository name.
- Select an open-source license and confirm its actual copyright holder. MIT is
  proposed in DECISIONS.md but has not been selected; this archive grants no license.
- Name the maintainer and independent contract reviewers.

## Publish and verify

1. Create the selected repository with main as its default branch. Upload this
   archive's stellarwork directory contents, including Cargo.lock,
   rust-toolchain.toml, and .github. Exclude target and Python caches.
2. Run CI on main and verify the model and contract jobs pass. Local validation
   does not establish a successful GitHub Actions run.
3. Configure a branch ruleset requiring pull requests, review, and the model and
   contract checks. The included workflow does not set repository rules.
4. Add the selected LICENSE before accepting external code contributions.
5. Publish issue drafts with their prerequisites intact and replace SW identifiers
   with actual GitHub links. Complete G0 and the remaining SW-001 CI checks first.
6. The first contract verification tasks are SW-002 (authorization), SW-003
   (rollback), and SW-004 (TTL). SW-005 (model fixtures) needs G0 only.

Do not assign contributors, deploy a contract, or advertise Wave acceptance merely
because this archive builds. Those actions require the actual repository, reviewers,
and deployment/program evidence described in READINESS.md.
