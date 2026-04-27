---
type: module
title: "Toast Notification Backend"
created: 2026-04-22
updated: 2026-04-22
tags:
  - rust
  - notifications
status: implemented
related:
  - "[[Windows Toast Click-To-Focus]]"
  - "[[Microsoft Toast Notification System]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Toast Notification Backend

The toast backend extends Windows notifications with click arguments. For focusable terminal notifications, it carries the pane identifier back into BENTERM when the toast is clicked.

Current behavior:

- Focusable toasts carry `focus-pane:<pane_id>` activation arguments.
- Focusable toasts use a fresh tag per agent-ready event and a pane-scoped group for cleanup.
- Focusable agent-ready toasts use `scenario="reminder"` and a `Focus` action button to make the Windows banner as sticky/nagging as the platform allows.
- Windows toast audio is silent so BENTERM's own sound cue stays distinct.
- The GUI removes a pane's outstanding toast group when the user responds in that pane.

Research note:

- Microsoft treats `Tag` plus `Group` as the primary key for replacement/removal. Stable per-pane tags make repeat completions behave like replacements. Fresh event tags plus stable pane groups are a better fit for BENTERM's nagging notification model. See [[Microsoft Toast Notification System]].
