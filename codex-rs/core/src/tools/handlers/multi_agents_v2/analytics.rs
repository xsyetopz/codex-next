use codex_analytics::AnalyticsEventsClient;
use codex_protocol::ThreadId;
use codex_protocol::items::CollabAgentTool;
use codex_protocol::items::CollabAgentToolCallItem;
use codex_protocol::items::CollabAgentToolCallStatus;

use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use crate::tools::context::ToolInvocation;
use crate::turn_timing::now_unix_timestamp_ms;

/// Records collaborator analytics and emits a metadata-only item publicly.
pub(super) struct ToolCallAnalytics {
    client: AnalyticsEventsClient,
    turn_id: String,
    item: CollabAgentToolCallItem,
    started_at_ms: i64,
    session: std::sync::Arc<Session>,
    turn: std::sync::Arc<TurnContext>,
}

impl ToolCallAnalytics {
    pub(super) fn new(invocation: &ToolInvocation, tool: CollabAgentTool) -> Self {
        Self {
            client: invocation.session.services.analytics_events_client.clone(),
            turn_id: invocation.turn.sub_id.clone(),
            item: CollabAgentToolCallItem {
                id: invocation.call_id.clone(),
                tool,
                status: CollabAgentToolCallStatus::Interrupted,
                sender_thread_id: invocation.session.thread_id,
                receiver_thread_ids: Vec::new(),
                receiver_agents: Vec::new(),
                prompt: None,
                model: None,
                reasoning_effort: None,
                resolved_model: None,
                resolved_reasoning_effort: None,
                agents_states: Default::default(),
            },
            started_at_ms: now_unix_timestamp_ms(),
            session: invocation.session.clone(),
            turn: invocation.turn.clone(),
        }
    }

    pub(super) fn set_receiver(&mut self, thread_id: ThreadId) {
        self.item.receiver_thread_ids = vec![thread_id];
    }

    pub(super) async fn finish(mut self, succeeded: bool) {
        self.item.status = if succeeded {
            CollabAgentToolCallStatus::Completed
        } else {
            CollabAgentToolCallStatus::Failed
        };
        // Keep the operation distinct from an existing SubAgentActivity item that uses the
        // tool call ID. The analytics event retains the original call ID for correlation.
        self.session
            .emit_turn_item_completed(
                &self.turn,
                codex_protocol::items::TurnItem::CollabAgentToolCall(public_item(
                    self.item.clone(),
                )),
            )
            .await;
        self.client.track_collab_tool_call(
            self.turn_id.clone(),
            self.item.clone(),
            self.started_at_ms,
            now_unix_timestamp_ms(),
        );
    }
}

pub(super) fn public_item(mut item: CollabAgentToolCallItem) -> CollabAgentToolCallItem {
    item.id = format!("{}::collab", item.id);
    item
}
