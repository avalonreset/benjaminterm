---
type: decision
title: "Keep Toast Click-To-Focus"
created: 2026-04-22
updated: 2026-04-22
tags:
  - notifications
  - decision
status: accepted
related:
  - "[[Windows Toast Click-To-Focus]]"
  - "[[Attention Pulse]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "cargo check -p wezterm-gui"
---

# Keep Toast Click-To-Focus

## Decision

Keep Windows toast click-to-focus as a BENTERM feature.

## Rationale

The feature directly solves the multi-window agent workflow problem: a completed agent can bring the user back to the exact terminal that needs input.

## Consequences

This is a source-level customization, so it must stay narrow and well-tested.

