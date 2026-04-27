---
type: concept
title: "Attention Pulse"
created: 2026-04-22
updated: 2026-04-22
tags:
  - notifications
  - visual-attention
status: implemented
related:
  - "[[Windows Toast Click-To-Focus]]"
  - "[[Tab Attention Indicator]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "cargo check -p wezterm-gui; cargo build -p wezterm-gui"
---

# Attention Pulse

Attention Pulse is a brief theme-accent edge pulse that appears when an agent-ready notification is received, and again after a toast notification is clicked. It makes the containing terminal window visually obvious when multiple BENTERM windows are visible.

Design constraints:

- Brief, not persistent.
- Drawn by BENTERM, not dependent on Windows taskbar behavior.
- Does not touch text shaping or terminal content in the current prototype.
- Implemented in the existing border painter to keep renderer risk low.
- Uses a saturated ANSI accent from the active color palette rather than foreground gray/white.
- The text/content wash idea is still a candidate if border plus tab attention is not visible enough across real themes.

Layering rule:

- The containing OS window gets the theme-accent edge pulse.
- The originating background tab gets [[Tab Attention Indicator]].
- Toast click retriggers the visual cue without replaying the sound.
- Agent-ready events also play the pane's sound identity from [[Sound Grab Bag Attention System]].
