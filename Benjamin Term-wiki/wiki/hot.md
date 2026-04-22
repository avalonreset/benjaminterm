---
type: meta
title: "Hot Cache"
created: 2026-04-22
updated: 2026-04-22
tags:
  - benjaminterm
  - cache
status: active
---

# Hot Cache

## Last Updated

2026-04-22

## Key Recent Facts

- Rebuild baseline is upstream WezTerm `577474d89`.
- BenjaminTerm keeps 0xProto, theme shuffle-bag, larger default scale, fancy tabs, BEN branding, and Windows toast click-to-focus.
- Toast click-to-focus now routes notification clicks back to the pane/window that emitted the alert.
- A short theme-accent attention pulse identifies the containing window when an agent-ready notification fires or a toast is clicked.
- Kenney Interface Sounds was selected for the prototype sound library: 100 CC0 UI sounds.
- Agent-ready notifications now prototype immediate pulse + per-pane sound even when no toast is shown.
- Pavlovian sound cues are independent of focus state; the cue should still play when the pane/window is already active.
- Per-pane sounds now come from a runtime shuffle bag and persist until that pane closes.
- Background tabs need their own visual marker; [[Tab Attention Indicator]] captures the current tab-level design.
- BenjaminTerm suppresses Windows toasts from the already-focused pane, while preserving toasts for background tabs and other windows as a lightweight attention queue.
- Focusable Windows toasts are tagged per pane and dismissed when that pane receives the user's next input or paste.

## Recent Changes

- Created the Benjamin Term-wiki vault.
- Captured rebuild decisions and current release state.
- Added notes for toast focus, attention pulse, and sound grab-bag ideas.
- Converted the Kenney 100-sound CC0 interface pack to WAV for native Windows playback.
- Captured the pane/tab-level sound identity requirement and tab attention indicator.
- Captured the checkpoint release behavior: theme-aware pulse, per-pane sound identity, silent Windows toasts, click-to-focus, focused-pane toast suppression, and input-cleared toast reminders.

## Active Threads

- Build and launch a source-based preview terminal.
- Prepare release packaging.
- Design tasteful agent-attention sounds without making the terminal noisy.
- Decide whether tab attention fades automatically or persists until selected.
- Consider a future content-area wash/glow if the border/tab pulse ever proves too subtle across theme packs.
