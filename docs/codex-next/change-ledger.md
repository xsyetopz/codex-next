# Change ledger

This ledger distinguishes observations from implemented source behavior. The
source customization inventory and observability changes below are complete for
this fork. The package was assembled for direct smoke testing but was not
installed or released.

## Baseline build findings

### B1: Release workspace versions do not match the committed Cargo lockfile

`cargo metadata --locked --format-version 1` fails on the untouched pinned
baseline because the lockfile would need updating. The workspace declares
`0.154.0`, while 150 local package entries in `Cargo.lock` declare `0.0.0`.
Cargo regeneration changes those local package versions to `0.154.0`.

- Design: regenerate the lockfile with Cargo, verify that no dependency source,
  checksum, or third-party version changes, and run the repository's Bazel
  lockfile workflow before committing a packaging fix.
- Current state: Cargo regeneration is verified to change only those 150 local
  versions, with no third-party source, checksum, or version changes.
  `just bazel-lock-update` succeeded without changing `MODULE.bazel.lock`.
- Rollback: reverse only the reviewed local-package version changes. Do not
  restore a lockfile containing later unrelated edits.

### B2: Bare workspace tests request an unavailable default V8 artifact

The approved `just test` run fails before executing tests. The `v8` crate requests
`librusty_v8_ptrcomp_sandbox_release_aarch64-apple-darwin.a.gz` from the
`denoland/rusty_v8` release `v150.4.0`; that asset returns HTTP 404.

This is not evidence that the sandbox must be disabled or that the source is
unbuildable. The repository already supplies the supported artifact-resolution
path in `scripts/codex_package/v8.py`:

- `resolve_codex_v8_cargo_env` selects the Codex-built archive and bindings.
- The helper verifies their checksum manifest against the pinned repository's
  trusted manifest, then verifies both artifacts.
- Both `RUSTY_V8_ARCHIVE` and `RUSTY_V8_SRC_BINDING_PATH` must be supplied together.
- The matching pair was fetched and verified successfully. The full suite was
  restarted with that pair, without disabling the V8 sandbox.

The repository generator helper is used as-is; no test workflow patch weakens
the sandbox or substitutes an unverified mirror.

### V1: Resumed subagent previews lose already-loaded activity

Confirmed on the pinned implementation: `AgentStatusThreadPreview::from_store`
only scans live buffered notifications. `ThreadEventStore` moves those items
into its turn snapshot during refresh. A resumed agent therefore displays no
recent activity even when its loaded history contains that activity.

- Implemented: inspect live items followed by the existing loaded history,
  newest first. Deduplicate by `(turn_id, item_id)` and retain at most six
  meaningful summaries. Prefer live content when both sources contain the item.
- Privacy and cost: borrow existing items; do not fetch history, send model
  requests, wake agents, or persist another transcript. Keep the existing
  240-grapheme summary and three-line display limits. Raw reasoning remains
  excluded.
- Reproduction: the new resumed-history regression test fails on the baseline
  with `[]` instead of the recovered activity. It passes after the fix and
  verifies that replay does not duplicate the summary. A TUI snapshot covers
  the resulting output.
- Focused checks: all three `agent_status` tests pass, including the existing
  bounded-output and reasoning-privacy fixtures.
- Focused TUI checks cover bounded activity, replay deduplication, requested and
  resolved model display, fast-mode status, and server settings updates. The one
  intentional activity snapshot was reviewed individually. Unrelated generated
  `.snap.new` files were not accepted.
- Compatibility: no wire shape, model context, or persisted history changes.
- Rollback: revert the two `agent_status_feed` Rust files together with this
  ledger entry; keep unrelated agent-navigation behavior unchanged.

### Completed source and package checks

- Fork clone and pinned upstream fetch succeeded.
- Local `next/0.154.0` points to the exact upstream commit; fork `main` is intact.
- Regenerated-lockfile `cargo metadata --locked --format-version 1` succeeded.
- `just test -p codex-tui multi_agents`: six tests passed, 4,314 skipped.
  This is a baseline check, not proof of new observability functionality.
- The local package smoke check assembled package version `0.154.0-next`; its
  CLI retains the pinned `0.154.0` identity. It was not installed or released.
- The pinned `just write-app-server-schema` recipe names a missing binary. The
  repository's schema-fixture script generated the checked-in app-server
  fixtures instead; this is a baseline tooling limitation, not a replacement
  Just recipe.

## Existing audit findings reused

These findings identify boundaries, not newly confirmed client bugs:

| Finding                                                             | Boundary and action                                                                                                |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Role sandbox declarations do not constrain inherited permissions    | Preserve parent permission enforcement; do not claim role prose creates isolation.                                 |
| Experimental context opt-in is conditional                          | Keep capability, authentication, provider, and activation checks; do not promise successful note persistence.      |
| Disabling the V2 flag does not force V1                             | Trace catalog selection and retained thread metadata; do not reinterpret a default-disabled flag as a V1 override. |
| Schema-valid service tier does not prove route eligibility          | Preserve catalog and provider gating; do not infer subscription savings.                                           |
| Historical command failures and repeated waits are detector signals | Do not treat every signal as a source defect or reopen private transcript analysis.                                |

## Completed V2 visibility design

The baseline already has `SubAgentActivityItem`, `SubAgentActivityEvent`, TUI
`AgentNavigationState`, and bounded `AgentStatusThreadPreview` views. App-server
already provides paginated `thread/items/list` and experimental
`thread/timeline/list`, plus descendant filtering on `thread/list`.

The fork extends these existing mechanisms rather than introducing a second
orchestrator. V2 public operation items use a derived `::collab` item ID so
they coexist with same-call `SubAgentActivity` records; private analytics keeps
the original call ID for correlation. `resolvedModel` and
`resolvedReasoningEffort` are absent when unknown. Public items omit encrypted
message content and hidden reasoning. No additional inference calls occur.

## Final validation evidence

- The final Rust 1.95.0 `just test` run executed 17,599 tests: 17,512 passed,
  87 failed, and 47 were skipped. Three tests recovered on retry and one was
  reported leaky. The earlier post-change run had 99 failures; all 12 failures
  tied to the changed fork default, public collaboration items, requested versus
  resolved metadata, fast-mode default, and their fixtures are green in the
  final run.
- The remaining failures are retained rather than normalized: executor and
  selected-capability environment fixtures, missing CLI helper/runtime setup,
  Guardian fixtures, sandbox/platform behavior, remote MCP timing, unrelated
  TUI snapshot drift, V8 proof-of-concept linkage, and native audio decoding.
  They are not evidence for a change in this remediation and are not claimed as
  fixed.
- Scoped Clippy repair completed for the affected crates, followed by repository
  formatting. The baseline unused `body_json` import remains intentionally
  untouched because it is unrelated work.
- The final package manifest reports `0.154.0-next`; the CLI reports
  `codex-cli 0.154.0`. An isolated config load showed `fast_mode=false` and
  `multi_agent_v2=true`. App-server initialization and `thread/list` succeeded,
  and the helper binary and daemon-wide `agents` interface exposed their help.
- Package SHA-256 values: `codex`
  `1c3f74fb6e8edb58b648bca87de60ac34f4f8bf51fb503f8db44360c06779754`;
  `codex-code-mode-host`
  `6916bf907e79d2dc24b0b6b85fe5705ec149b9b7826c9305f49b8744c4861f1c`;
  bundled `rg`
  `a326a1fb48074202e9ad41e4cd1e389eeea372c8c6f7d7e80da81176d5d9430e`;
  package manifest
  `18b6e3d50d49882d145df0b4b22ffe0972eced1be30b8f1053957f8acf5ccf48`.
