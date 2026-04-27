---
type: meta
title: "Hot Cache"
created: 2026-04-22
updated: 2026-04-22
tags:
  - benterm
  - cache
status: active
---

# Hot Cache

## Last Updated

2026-04-22

## Key Recent Facts

- Rebuild baseline is upstream WezTerm `577474d89`.
- BENTERM keeps 0xProto, theme shuffle-bag, larger default scale, fancy tabs, BEN branding, and Windows toast click-to-focus.
- Toast click-to-focus now routes notification clicks back to the pane/window that emitted the alert.
- A short theme-accent attention pulse identifies the containing window when an agent-ready notification fires or a toast is clicked.
- BENTERM now uses a fully re-sourced softer 84-file CC0 cue set derived from Kenney UI Audio plus ObsydianX Interface SFX Pack 1; see [[Soft Cue Pack Refresh]].
- `BENTERM-windows-v1.4.0.zip` and `BENTERM-windows-v1.4.1.zip` each contain 84 `benterm-soft-cues` WAVs and 0 old `kenney-interface` entries.
- Only attribute sound sources that ship. Current shipped sound attribution is Kenney UI Audio plus ObsydianX Interface SFX Pack 1.
- The soft cue pack is per-file peak normalized around `-14 dB`; packaged measurement is quietest `-14.0 dB`, loudest `-13.9 dB`.
- Current audio review folder: `dist/BENTERM-windows-v2026.04.22-soft-cues-curated/sounds/benterm-soft-cues/`.
- Agent-ready notifications now prototype immediate pulse + per-pane sound even when no toast is shown.
- Pavlovian sound cues are independent of focus state; the cue should still play when the pane/window is already active.
- Per-pane sounds now come from a runtime shuffle bag and persist until that pane closes.
- Background tabs need their own visual marker; [[Tab Attention Indicator]] captures the current tab-level design.
- BENTERM suppresses Windows toasts from the already-focused pane, while preserving toasts for background tabs and other windows as a lightweight attention queue.
- Focusable Windows toasts now use fresh event tags with pane-scoped groups; this avoids same-pane ready events becoming silent replacements.
- Focusable agent-ready toasts use Windows `scenario="reminder"` plus a `Focus` action button for stronger nagging behavior.
- Pane input clears that pane's entire outstanding toast group.
- [[Idle Text Glow Cue]] is accepted for release as an event-armed, one-shot breathing cursor-row cue. It clears on user input, uses a hard-edged pane-painted band, and selects a theme accent that avoids the cursor color.
- Current release package is `dist/BENTERM-windows-v1.4.1.zip`.
- Current GitHub release is `https://github.com/avalonreset/BENTERM/releases/tag/v1.4.1`.
- GitHub Releases are the official distribution channel for Windows, macOS, and Linux. Package-manager channels are not required for release completion.
- Current preview emits OSC 777 after two seconds so the same ready path as sound/border/glow is exercised.
- User review accepted the breathing pulse behavior, then requested cleaner hard edges and a non-cursor color. The release implementation removes fuzzy edge/aura rectangles and scores palette accents away from cursor colors.
- Installed upstream WezTerm is unrelated to this work and should not be used for BENTERM feature review.
- Installed BENTERM windows under `C:\Program Files\BENTERM\` may still be open, but they are release/install-state windows, not the source preview for the current glow work.
- Raw `wezterm-gui.exe` source previews are no longer acceptable for BENTERM feature review.
- `wezterm-gui/build.rs` now emits BENTERM Windows resource metadata, and `assets/windows/terminal.ico` was refreshed from the newer BENTERM logo icon.

## Recent Changes

- Created the BENTERM-wiki vault.
- Captured rebuild decisions and current release state.
- Added notes for toast focus, attention pulse, and sound grab-bag ideas.
- Replaced the old Kenney Interface Sounds cue library with a fully re-sourced 84-file CC0 soft cue pack for native Windows playback.
- Removed 16 extremely low-energy sounds from the replacement set and rebuilt `dist/BENTERM-windows-v2026.04.22-soft-cues-curated.zip`.
- Local installer rebuild is no longer blocked; Inno Setup is installed at `C:/Users/rccol/AppData/Local/Programs/Inno Setup 6/ISCC.exe`.
- Captured the pane/tab-level sound identity requirement and tab attention indicator.
- Captured the checkpoint release behavior: theme-aware pulse, per-pane sound identity, silent Windows toasts, click-to-focus, focused-pane toast suppression, and input-cleared toast reminders.
- Replaced the rejected hard halo glow and too-subtle all-text aura with an event-armed pane-painted cursor-row cue that breathes through a dedicated animation loop.

## Active Threads

- Release wrap-up: `v1.4.1` is latest, with the no-space `BENTERM` installer naming patch.
- Local install remains `v1.4.0` until the user chooses to run the patch installer.
- Continue user ear review of [[Soft Cue Pack Refresh]].
- Keep agent-attention sounds quiet, short, and non-startling during long coding sessions.
- Decide whether tab attention fades automatically or persists until selected.
- Branding follow-up: verify installed EXE metadata/icon and taskbar identity from `C:\Program Files\BENTERM\BENTERM-gui.exe`.
- Local install verification passed: installed `BENTERM-gui.exe` hash matches the v1.4.0 package, metadata is `BENTERM / Avalon Reset`, `sounds/benterm-soft-cues` has 84 WAV files, and the old `sounds/kenney-interface` folder is gone.
- Pinned taskbar shortcut was updated to target `C:\Program Files\BENTERM\BENTERM-gui.exe` and use icon `C:\Program Files\BENTERM\BENTERM-gui.exe,0`; remaining old-looking taskbar icon behavior is expected to be Windows icon cache/shortcut cache.
- Patch release `v1.4.1` is live at `https://github.com/avalonreset/BENTERM/releases/tag/v1.4.1`; it corrects the installer AppName and shell labels from `BENTERM` to `BENTERM` so Start/taskbar surfaces do not use the spaced brand.
- Archived public fork `BENTERM-fork-archive-2026-04-22` was deleted manually by the user; only active public BENTERM repo should remain.
- Manually verify reminder-toast behavior across Focus Assist states and Windows notification settings.
- Public install docs now live in `INSTALL.md`.
