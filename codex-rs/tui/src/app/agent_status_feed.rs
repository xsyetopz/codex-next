//! Bounded, best-effort previews for the v2 `/subagents` status output.

use super::ThreadBufferedEvent;
use super::ThreadEventStore;
use crate::history_cell::HistoryCell;
use crate::history_cell::plain_lines;
use crate::multi_agents::AGENT_ACTIVITY_PREVIEW_ITEMS;
use crate::multi_agents::agent_activity_summary;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadItem;
use ratatui::style::Stylize;
use ratatui::text::Line;
use std::collections::HashSet;

const AGENT_STATUS_PREVIEW_LINES: usize = 3;
const AGENT_STATUS_PREVIEW_INDENT: u16 = 4;

#[derive(Debug)]
pub(super) struct AgentStatusHistoryCell {
    entries: Vec<AgentStatusThreadPreview>,
}

impl AgentStatusHistoryCell {
    pub(super) fn new(entries: Vec<AgentStatusThreadPreview>) -> Self {
        Self { entries }
    }
}

impl HistoryCell for AgentStatusHistoryCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = vec![
            "/subagents".magenta().into(),
            "Sub-agents running".bold().into(),
            "".into(),
        ];

        if self.entries.is_empty() {
            lines.push("  • No sub-agents running.".italic().into());
            return lines;
        }

        for entry in &self.entries {
            lines.push(entry.title_line());
            let preview_width = width.saturating_sub(AGENT_STATUS_PREVIEW_INDENT).max(1);
            let preview_lines = entry.preview_lines(preview_width);
            if preview_lines.is_empty() {
                lines.push(vec!["    ".into(), "No recent activity yet.".dim().italic()].into());
            } else {
                lines.extend(preview_lines.into_iter().map(indent_preview_line));
            }
            lines.push("".into());
        }
        let _ = lines.pop();
        lines
    }

    fn raw_lines(&self) -> Vec<Line<'static>> {
        plain_lines(self.display_lines(u16::MAX))
    }
}

#[derive(Debug)]
pub(super) struct AgentStatusThreadPreview {
    agent_path: String,
    activity: Vec<String>,
}

impl AgentStatusThreadPreview {
    pub(super) fn from_store(agent_path: String, store: &ThreadEventStore) -> Self {
        let live_items = store.buffer.iter().rev().filter_map(|event| match event {
            ThreadBufferedEvent::Notification(notification) => match notification.as_ref() {
                ServerNotification::ItemCompleted(event) => {
                    Some((event.turn_id.as_str(), &event.item))
                }
                ServerNotification::ItemStarted(event) => {
                    Some((event.turn_id.as_str(), &event.item))
                }
                _ => None,
            },
            ThreadBufferedEvent::Request(_)
            | ThreadBufferedEvent::HistoryEntryResponse(_)
            | ThreadBufferedEvent::FeedbackSubmission(_) => None,
        });
        // Refresh moves activity from the live buffer into the existing turn snapshot.
        // Borrow that history rather than retaining a second transcript or fetching it again.
        let history_items = store.turns.iter().rev().flat_map(|turn| {
            turn.items
                .iter()
                .rev()
                .map(move |item| (turn.id.as_str(), item))
        });
        Self::from_items(agent_path, live_items.chain(history_items))
    }

    pub(super) fn empty(agent_path: String) -> Self {
        Self::from_items(agent_path, std::iter::empty())
    }

    fn from_items<'a>(
        agent_path: String,
        items: impl Iterator<Item = (&'a str, &'a ThreadItem)>,
    ) -> Self {
        let mut seen_item_ids = HashSet::new();
        let mut activity = Vec::new();
        for (turn_id, item) in items {
            let Some(summary) = agent_activity_summary(item) else {
                continue;
            };
            if seen_item_ids.insert((turn_id, item.id())) {
                activity.push(summary);
                if activity.len() == AGENT_ACTIVITY_PREVIEW_ITEMS {
                    break;
                }
            }
        }
        activity.reverse();
        Self {
            agent_path,
            activity,
        }
    }

    fn title_line(&self) -> Line<'static> {
        vec!["  • ".dim(), format!("`{}`", self.agent_path).cyan()].into()
    }

    fn preview_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = self
            .activity
            .iter()
            .flat_map(|activity| textwrap::wrap(activity, width as usize))
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.into_owned().dim().into())
            .collect::<Vec<_>>();
        if lines.len() > AGENT_STATUS_PREVIEW_LINES {
            lines.drain(..lines.len() - AGENT_STATUS_PREVIEW_LINES);
        }
        lines
    }
}

fn indent_preview_line(mut line: Line<'static>) -> Line<'static> {
    line.spans.insert(0, "    ".into());
    line
}

#[cfg(test)]
#[path = "agent_status_feed_tests.rs"]
mod tests;
