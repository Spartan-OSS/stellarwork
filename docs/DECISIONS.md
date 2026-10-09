# Decisions and proposed defaults

| Decision | Default | Reason |
| --- | --- | --- |
| Project name | StellarWork, provisional | recognizable working name; availability not checked |
| Initial asset | one reviewed SAC per deployment | narrows token/accounting risk |
| First environment | testnet/local only | establish the complete flow before payment pilots |
| Milestones | independent; cap 20 | bounded storage/keep-alive footprint |
| Terms acceptance | all three roles authorize creation | neither worker nor judge silently enrolled |
| Arbitration | agreed individual, no fees/bonds | smallest coherent dispute mechanism |
| Deadline | full refund only if Funded and overdue | submitted work must remain eligible for judgment |
| Evidence | bounded URI; private documents off-chain | avoid publishing customer deliverables |
| Storage | persistent obligations; instance metadata | independent milestone keys and retained liabilities |
| License | MIT proposed; confirm before public contributions | permissive ecosystem integration |
| Deployment operations | maintainer-owned initial setup | matches Goodness's infrastructure/CI background |

## Gate decisions to record before assigning dependent work

1. Confirm the organization/repository and open-source license; include actual ownership
   details in LICENSE. Do not invent GitHub organization or reviewer usernames.
2. Confirm SDK/toolchain from a green native build and record exact versions and lockfile.
3. Confirm three-party authorization collection UX on testnet before general frontend work.
4. Decide arbitrator nonresponse policy, retention horizon, and evidence privacy agreement
   before any external pilot. No contributor should invent those policies in a feature PR.
5. Define a testnet token's issuer/address/decimals from its deployment; no assumed USDC
   addresses or production token claims.

A useful next product-validation step: show the happy path and dispute scenario to
3–5 remote contractors/clients, ask who they would accept as arbitrator, and record
whether signatures/evidence handling are practical. These interviews are not yet done.
