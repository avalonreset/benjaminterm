---
type: module
title: "Renderer Attention Pulse"
created: 2026-04-22
updated: 2026-04-22
tags:
  - renderer
  - visual-attention
status: implemented
related:
  - "[[Attention Pulse]]"
  - "[[Idle Text Glow Cue]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Renderer Attention Pulse

Renderer Attention Pulse draws the short edge/window cue used by agent-ready notifications and toast click focus.

Related renderer work now includes [[Idle Text Glow Cue]], which is intentionally separate from the border pulse:

- The border pulse is event-driven and short.
- The text glow is armed by the ready event and clears on user input.
- The text glow is implemented in the glyph quad path as a smooth foreground color mix; the offset halo prototype was rejected as too visually distorted.

