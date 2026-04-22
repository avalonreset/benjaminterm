use mux::pane::PaneId;

pub(crate) const AGENT_READY_TOAST_GROUP: &str = "benjaminterm-agent-ready";

pub(crate) fn tag_for_pane(pane_id: PaneId) -> String {
    format!("pane-{pane_id}")
}

pub(crate) fn dismiss_for_pane(pane_id: PaneId) {
    wezterm_toast_notification::dismiss_toast_notification(
        &tag_for_pane(pane_id),
        Some(AGENT_READY_TOAST_GROUP),
    );
}
