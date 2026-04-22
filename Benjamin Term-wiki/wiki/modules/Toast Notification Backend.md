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
sources:
  - "[[Rebuild Session Summary]]"
---

# Toast Notification Backend

The toast backend extends Windows notifications with click arguments. For focusable terminal notifications, it carries the pane identifier back into BenjaminTerm when the toast is clicked.

Current behavior:

- Focusable toasts carry `focus-pane:<pane_id>` activation arguments.
- Focusable toasts are tagged per pane under the `benjaminterm-agent-ready` group.
- Windows toast audio is silent so BenjaminTerm's own sound cue stays distinct.
- The GUI can remove a pane's outstanding toast when the user responds in that pane.
