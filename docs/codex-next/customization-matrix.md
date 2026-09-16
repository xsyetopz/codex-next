# Customization matrix

Upstream merge tip: `50d77959bf927293c4b5ddcca81d05331ae582ea`.
This inventory records client-owned controls only; service eligibility and host
rendering remain external boundaries.

| Surface | Controls and precedence | Consumer | Ownership / stability | Validation and rollback |
| --- | --- | --- | --- | --- |
| Configuration | `config.toml`, profiles, managed policy; managed policy wins over local configuration | `codex-core` configuration and session construction | Framework / mixed, stable by documented schema | `just write-config-schema`; remove the local key to roll back |
| Fast mode | `features.fast_mode`; disabled when omitted, enabled only by an explicit `true` | feature registry, session and TUI service-tier resolution | Source-fork default change; stable feature | feature default regression test; set `true` explicitly to opt in |
| Multi-agent V2 | `multi_agent_v2`, role definitions, `spawn_agent` arguments; role/runtime resolution follows requested settings | `core::tools::handlers::multi_agents_v2` | Harness modification, experimental | focused core tests; disable V2 or revert the patch |
| CLI and environment | CLI options and environment are parsed before session configuration | CLI/session startup | Framework, stable only where documented | `codex --help`; remove flags/environment |
| App-server | V2 `thread/items/list`, timeline and notifications expose persisted public items | `app-server-protocol::protocol::v2` | Framework plus this additive wire change, experimental | schema generation and protocol tests; clients ignore additive fields |
| Extensions / MCP | configured extension and MCP discovery, capability and approval gates | extension registry and connection manager | Mixed / capability-dependent | existing extension tests; remove configured extension |
| External host/service | Desktop rendering, account tier, model catalog and encrypted communication | host application and backend | External / unverified | no client-side guarantee; use host/service rollback procedures |

## Collaboration observability

`CollabAgentToolCall` retains requested model and effort separately from
`resolvedModel` and `resolvedReasoningEffort`. Missing values mean that no
recipient configuration applied or it was not observable; they do not mean an
empty setting. Public V2 operation items deliberately omit prompts and other
encrypted communication content. Accessible activity uses the existing turn
items and bounded TUI previews, not a new transcript, inference request, or
agent wakeup.

V2 `fork_turns` treats omitted or whitespace-only input as fresh context.
Explicit `all` and positive values retain their established fork behavior; V1
continues to use its independent `fork_context` contract.
