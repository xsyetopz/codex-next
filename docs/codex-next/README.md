# Codex next

Source fork with a locally assembled and smoke-tested package. It has not been
installed or released. See the
[customization matrix](customization-matrix.md), [change ledger](change-ledger.md),
and [operator guide](operator-guide.md).

## Provenance

- Upstream merge tip: `50d77959bf927293c4b5ddcca81d05331ae582ea`.
- Development branch: `next/0.154.0`.
- Existing fork `main`: `ee6814bfa4889fe9b2b3dcc9cc8bdd91effa8ab8`, preserved.
- Issue search cutoff: reports created through 2026-09-14 UTC. Search results
  are candidates, not confirmed defects. A search page is not complete coverage.
- Workspace package version: `0.154.0-next`. This is the fork's retained local
  label and does not identify the merged upstream revision.

## Ownership boundaries

```mermaid
flowchart LR
    Framework[Configuration and extensions] --> Client[Public Rust client]
    Client --> TUI[CLI and TUI]
    Client --> Protocol[App-server protocol]
    Protocol --> Host[External host application]
    Client --> Service[Model and account services]
```

Configuration affects client behavior within the implemented permission and
capability checks. A public client patch cannot guarantee Desktop rendering,
model behavior, subscription accounting, encrypted-content availability, or
backend entitlement. Feature stability and default enablement are independent
of ownership and of demonstrated runtime success.

## Installation gate

Do not switch the default launcher until the modified package, its helper
binaries, compatibility checks, and rollback have been verified. Do not replace
Desktop application binaries. Do not change live configuration to compensate
for an unverified build.

The original Bun launcher and installed binaries remain in place. Local build
logs and packages are kept outside this repository under
`~/.local/state/codex-next/`; they must not be copied into a published branch.
That directory contains evidence, not a replacement for native execution state.

See the [change ledger](change-ledger.md) for actual checks and limitations.
