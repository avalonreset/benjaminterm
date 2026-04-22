---
type: concept
title: "Tab Attention Indicator"
created: 2026-04-22
updated: 2026-04-22
tags:
  - tabs
  - notifications
  - visual-attention
status: prototyping
related:
  - "[[Attention Pulse]]"
  - "[[Sound Grab Bag Attention System]]"
  - "[[Agent Completion Attention Flow]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "cargo check -p wezterm-gui; cargo build -p wezterm-gui"
---

# Tab Attention Indicator

Tab Attention Indicator marks the tab that emitted an agent-ready notification. It is needed because a user may run multiple Codex or Claude sessions as tabs inside one BenjaminTerm window instead of separate OS windows.

Design intent:

- Do not throw away WezTerm's tab system.
- Do not force focus to a background tab just because it completed.
- Make the finished tab discoverable at a glance.
- Keep the first prototype in the fancy tab bar paint path, not terminal text shaping.

Prototype behavior:

- The originating pane is resolved to its containing tab.
- The containing tab receives a short theme-accent pulse in the fancy tab bar.
- The same attention event also triggers the window edge pulse and the per-pane sound cue.

Open refinements:

- Decide whether the tab marker should persist until selected or fade automatically.
- Decide whether split panes inside one tab should share a tab cue but keep separate sounds.
- Consider a subtle content glow only after the lower-risk border/tab system is stable.
