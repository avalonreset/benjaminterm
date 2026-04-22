use mux::pane::PaneId;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TOAST_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) fn group_for_pane(pane_id: PaneId) -> String {
    format!("benjaminterm-agent-ready-pane-{pane_id}")
}

pub(crate) fn tag_for_ready_event(pane_id: PaneId) -> String {
    let id = NEXT_TOAST_ID.fetch_add(1, Ordering::Relaxed);
    format!("ready-{pane_id}-{id}")
}

pub(crate) fn dismiss_for_pane(pane_id: PaneId) {
    wezterm_toast_notification::dismiss_toast_notification_group(&group_for_pane(pane_id));
}
