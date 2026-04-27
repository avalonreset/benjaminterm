---
type: module
title: "Terminal Defaults"
created: 2026-04-22
updated: 2026-04-23
status: active
tags:
  - config
  - defaults
  - notifications
related:
  - "[[BENTERM]]"
  - "[[Notification Noise Policy]]"
  - "[[Smart Copy Interrupt]]"
  - "[[BENTERM and Hyper Yap Boundary]]"
---

# Terminal Defaults

BENTERM keeps WezTerm's terminal engine while changing selected defaults for its own distribution experience.

## Notification Defaults

- BENTERM suppresses missing-glyph warnings by default.
- Focused-pane notifications use the BENTERM default notification handling behavior.

## Smart Copy Defaults

- `Ctrl+C` copies and clears selection when text is selected.
- `Ctrl+C` sends interrupt input to the pane when no text is selected.

## Automation Boundary

BENTERM should not become the owner of personal dictation semantics. Hyper Yap should handle dictated-block insertion and undo because it knows which text came from speech-to-text automation.

## User-Facing Reason

The missing-glyph warning was surfacing as repeated desktop notifications for normal use. BENTERM should avoid noisy default notifications while still allowing power users to opt into upstream behavior through config.

## Important Files

- `config/src/config.rs`
- `docs/config/lua/config/notification_handling.md`
- `docs/config/lua/config/warn_about_missing_glyphs.md`
