---
type: decision
title: "Idle Text Glow Cue"
created: 2026-04-22
updated: 2026-04-22
tags:
  - benjaminterm
  - visual-attention
  - renderer
status: accepted
related:
  - "[[Renderer Attention Pulse]]"
  - "[[Agent Completion Attention Flow]]"
  - "[[Attention Pulse]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Idle Text Glow Cue

BenjaminTerm is experimenting with a subtle pulsing glow-color shift to terminal text after an agent-ready attention event. This is meant to answer the user-facing question: which terminal is inviting the next prompt?

Current status: accepted for `v1.4.0`. The first visible prototype used offset glyph halo quads and looked distorted. The second prototype removed the halo and made the effect event-armed, but a plain shell showed no visible glow because it did not emit the ready attention event. The third prototype produced a visible cursor-row highlight, but user review confirmed it was static. The accepted implementation uses a dedicated glow animation loop so the pane-painted cursor-row cue breathes independently of generic render scheduling, with hard edges and a theme accent selected away from the cursor color.

Implementation notes:

- Enabled by default through `benjaminterm_idle_text_glow`.
- Armed by `start_visual_attention_for_pane`, the same path used by ready sounds and the border/window pulse.
- Starts after `benjaminterm_idle_text_glow_delay_ms`, currently `500`, from the ready event.
- Clears on user input and stays off until the next ready event.
- Suppresses immediately after user input for `benjaminterm_idle_text_glow_input_suppression_ms`, currently `1200`, so a rapid follow-up ready event does not flash instantly.
- Uses `benjaminterm_idle_text_glow_period_ms`, currently `2600`, for a slow pulse.
- Uses `benjaminterm_idle_text_glow_strength`, currently `0.42`, clamped internally.
- Selects a saturated theme accent from the active palette and avoids colors close to `cursor_bg` or `cursor_border`.
- Mixes cursor-row glyph foreground toward the glow color. The hard offset halo prototype was rejected because it looked like doubled/distorted text rather than a soft glow.
- Adds a hard-edged low-alpha breathing band on the cursor row using the renderer's existing filled-rectangle path.
- Also paints the cursor-row glow directly at the pane level, after the pane background and before line rendering. This avoids depending solely on cached per-line quads for visibility.
- No longer suppresses glow when the window focus state is unavailable; the cue is tied to ready attention state, not focus bookkeeping.
- Owns a dedicated `50 ms` invalidation loop while the pane's `idle_text_glow_start` remains armed. This is intended to make the row highlight actually pulse instead of appearing as a static band.
- Schedules the breathing animation at at least `30 fps` even when the global `animation_fps` default is lower.
- Skips cursor cells, selected text, invisible text, and color emoji/image glyphs.
- Does not reactivate from ordinary typing/deleting; the cue arms only from the next ready attention event.

Current test build:

- Built from `E:\benjaminterm-sound-refresh`.
- Current preview executable: `target/debug/BenjaminTerm-gui.exe`, copied from the source-built GUI binary after build.
- Current spawned preview process: `40092`.
- Current preview command is `scripts/preview/launch-benjaminterm-glow-preview.ps1`; it writes `.tmp/benjaminterm-glow-trigger.ps1` and launches PowerShell with `-File` so OSC 777 testing does not depend on fragile inline quoting.
- Branding issue remains for release: the upstream crate still builds `wezterm-gui.exe`; `BenjaminTerm-gui.exe` is currently a post-build preview copy.

Known follow-ups:

- A real release/testing path should include a first-class BenjaminTerm executable name rather than relying on a copied preview binary.
- A durable local test trigger is still useful, preferably a debug-only action or command that calls the same attention path as an agent-ready notification.
- The glow must remain one-shot per ready event and clear on first user input.
- The glow must be subtle and readable, not a constant idle animation.

Validation:

- `cargo check -p wezterm-gui --quiet` passed with existing upstream warnings.
- `cargo build -p wezterm-gui` passed with existing upstream warnings.
- `.tmp/benjaminterm-glow-trigger.ps1` parsed and emitted OSC 777 without PowerShell token errors.
- Desktop screenshot verification showed the cursor-row glow rendered in the source preview window.
- User review accepted the breathing pulse behavior and requested hard edges plus a color distinct from the cursor; both were applied before the `v1.4.0` release build.
