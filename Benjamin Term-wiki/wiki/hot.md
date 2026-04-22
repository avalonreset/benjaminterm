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
- BenjaminTerm now uses a fully re-sourced softer 84-file CC0 cue set derived from Kenney UI Audio plus ObsydianX Interface SFX Pack 1; see [[Soft Cue Pack Refresh]].
- The soft cue pack is per-file peak normalized around `-14 dB`; packaged measurement is quietest `-14.0 dB`, loudest `-13.9 dB`.
- Current audio review folder: `dist/BenjaminTerm-windows-v2026.04.22-soft-cues-curated/sounds/benjaminterm-soft-cues/`.
- Agent-ready notifications now prototype immediate pulse + per-pane sound even when no toast is shown.
- Pavlovian sound cues are independent of focus state; the cue should still play when the pane/window is already active.
- Per-pane sounds now come from a runtime shuffle bag and persist until that pane closes.
- Background tabs need their own visual marker; [[Tab Attention Indicator]] captures the current tab-level design.
- BenjaminTerm suppresses Windows toasts from the already-focused pane, while preserving toasts for background tabs and other windows as a lightweight attention queue.
- Focusable Windows toasts now use fresh event tags with pane-scoped groups; this avoids same-pane ready events becoming silent replacements.
- Focusable agent-ready toasts use Windows `scenario="reminder"` plus a `Focus` action button for stronger nagging behavior.
- Pane input clears that pane's entire outstanding toast group.
- [[Idle Text Glow Cue]] is accepted for release as an event-armed, one-shot breathing cursor-row cue. It clears on user input, uses a hard-edged pane-painted band, and selects a theme accent that avoids the cursor color.
- Current release package is `dist/BenjaminTerm-windows-v1.4.0.zip`.
- Current preview emits OSC 777 after two seconds so the same ready path as sound/border/glow is exercised.
- User review accepted the breathing pulse behavior, then requested cleaner hard edges and a non-cursor color. The release implementation removes fuzzy edge/aura rectangles and scores palette accents away from cursor colors.
- Installed upstream WezTerm is unrelated to this work and should not be used for BenjaminTerm feature review.
- Installed BenjaminTerm windows under `C:\Program Files\BenjaminTerm\` may still be open, but they are release/install-state windows, not the source preview for the current glow work.
- Raw `wezterm-gui.exe` source previews are no longer acceptable for BenjaminTerm feature review.
- `wezterm-gui/build.rs` now emits BenjaminTerm Windows resource metadata, and `assets/windows/terminal.ico` was refreshed from the newer BenjaminTerm logo icon.

## Recent Changes

- Created the Benjamin Term-wiki vault.
- Captured rebuild decisions and current release state.
- Added notes for toast focus, attention pulse, and sound grab-bag ideas.
- Replaced the old Kenney Interface Sounds cue library with a fully re-sourced 84-file CC0 soft cue pack for native Windows playback.
- Removed 16 extremely low-energy sounds from the replacement set and rebuilt `dist/BenjaminTerm-windows-v2026.04.22-soft-cues-curated.zip`.
- Local installer rebuild is no longer blocked; Inno Setup is installed at `C:/Users/rccol/AppData/Local/Programs/Inno Setup 6/ISCC.exe`.
- Captured the pane/tab-level sound identity requirement and tab attention indicator.
- Captured the checkpoint release behavior: theme-aware pulse, per-pane sound identity, silent Windows toasts, click-to-focus, focused-pane toast suppression, and input-cleared toast reminders.
- Replaced the rejected hard halo glow and too-subtle all-text aura with an event-armed pane-painted cursor-row cue that breathes through a dedicated animation loop.

## Active Threads

- Finalize `v1.4.0` release packaging and installer.
- Install `v1.4.0` over the current local BenjaminTerm installation after validating branding.
- Continue user ear review of [[Soft Cue Pack Refresh]].
- Keep agent-attention sounds quiet, short, and non-startling during long coding sessions.
- Decide whether tab attention fades automatically or persists until selected.
- Branding follow-up: verify installed EXE metadata/icon and taskbar identity from `C:\Program Files\BenjaminTerm\BenjaminTerm-gui.exe`.
- Manually verify reminder-toast behavior across Focus Assist states and Windows notification settings.
