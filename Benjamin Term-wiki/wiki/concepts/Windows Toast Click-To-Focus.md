---
type: concept
title: "Windows Toast Click-To-Focus"
created: 2026-04-22
updated: 2026-04-22
tags:
  - notifications
  - windows
  - productivity
status: implemented
related:
  - "[[Attention Pulse]]"
  - "[[Agent Completion Attention Flow]]"
  - "[[Microsoft Toast Notification System]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "cargo check -p wezterm-toast-notification; cargo check -p wezterm-gui"
---

# Windows Toast Click-To-Focus

Windows toast click-to-focus routes a notification click back to the pane/window that emitted the alert.

Implementation shape:

- Terminal tools emit standard OSC 9 or OSC 777 toast notifications.
- BenjaminTerm attaches a `focus-pane:<pane_id>` click argument for focusable notifications.
- The Windows toast backend dispatches activation arguments back to the GUI.
- The GUI focuses the originating pane and containing window.
- Toasts are silent; BenjaminTerm owns the sound cue through [[Sound Grab Bag Attention System]].
- BenjaminTerm suppresses toasts from the already-focused pane, but keeps them for background tabs and other windows so they can act as reminders.
- Focusable toasts use a fresh tag for each ready event and a pane-scoped group. When that pane receives the user's next input or paste, BenjaminTerm removes that pane's outstanding toast group from Windows notification history.

This is a high-value source change because it directly supports multi-agent terminal workflows without modifying terminal text semantics.
