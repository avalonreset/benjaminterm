---
type: concept
title: "Attention Trigger Lifecycle"
created: 2026-04-27
updated: 2026-04-27
status: active
tags:
  - notifications
  - attention
  - sounds
  - osc
related:
  - "[[Sound Grab Bag Attention System]]"
  - "[[Notification Noise Policy]]"
  - "[[Terminal Defaults]]"
sources:
  - "[[Project Documentation]]"
---

# Attention Trigger Lifecycle

The attention features - per-pane border pulse, cursor-row idle glow, soft-cue sound - fire on a single signal: **OSC 9** (or **OSC 777**) emitted into a pane. This is the *agent ready signal* per the rankenstein-suite Mandatory Requirements (M1, M3). Anything else (BEL, idle-detection heuristics, prompt timers) is **not** the trigger and should not be wired in.

## Trigger Source

A pane fires the attention pipeline when its terminal output contains an OSC 9 (or OSC 777) escape sequence:

```
\x1b]9;<message>\x07
\x1b]777;<message>\x07
```

Codex emits OSC 9 natively when it finishes a turn. Plain shells and other agents do not - they need integration glue to emit it (see *Integrations* below).

## Pipeline (BENTERM Rust core)

1. Terminal parser receives OSC 9 → emits `Alert::ToastNotification`.
2. `wezterm-gui/src/frontend.rs` routes the alert to `trigger_attention_for_pane(window_id, pane_id)`.
3. `trigger_attention_for_pane`:
   - Calls `attention_sound::play_for_pane(pane_id)` - random Kenney UI cue from the per-pane shuffle-bag.
   - Calls `term_window.start_visual_attention_for_pane(pane_id)` - sets `bell_start`, `idle_text_glow_start`, `agent_attention_start`, starts the per-pane attention pulse and cursor-row glow animation.
4. `notification_handling` config gates **only** the Windows toast popup, **not** the per-pane in-terminal features. Toast is suppressible; attention features always fire.

## Anti-patterns (do not introduce)

- **Idle-detection on `PaneOutput`.** Streaming agents (Claude, Codex) emit dozens of small output events per turn with sub-second pauses between them. Any "fire if no output for N ms" timer turns one ready event into a constant click-track. Removed in favor of OSC 9.
- **BEL → attention.** BEL is the legacy visual_bell, not the ready signal. A program emitting BEL on tab-completion error does not mean "agent finished." Keep BEL on the upstream visual_bell render path; do not call `start_visual_attention_for_pane` from the BEL handler.
- **Forcing the glow off the cursor row.** The cursor row is geometrically correct. If a TUI hides the cursor visually, that is a TUI concern, not a render-path concern.

## Integrations (how each shell/agent ends up emitting OSC 9)

- **Codex** - emits OSC 9 natively. No glue required.
- **pwsh / bash plain shell** - bundle a shell-integration script that emits OSC 9 from the `prompt` function on command end. Lives in `assets/shell-integration/benterm.ps1` (pwsh). Sourced from `$PROFILE` or via `default_prog`.
- **Claude Code** - does not emit OSC 9 by default. Bridge it with a `Stop` hook in `~/.claude/settings.json`:
  ```json
  "hooks": {
    "Stop": [{
      "hooks": [{
        "type": "command",
        "command": "printf '\\033]9;BENTERM Ready\\007' > /dev/tty 2>/dev/null || true"
      }]
    }]
  }
  ```
  The hook writes OSC 9 directly to the controlling tty when Claude finishes responding. BENTERM receives it, fires per-pane attention.
- **Other agent CLIs** - same pattern: emit OSC 9 on turn end via whatever hook/wrapper that CLI exposes.

## Dismissal

The cursor-row glow clears in `mark_pane_input` when the user types. `last_input.replace(now)`, `idle_text_glow_start.take()`, animation deactivated. The border pulse runs its own short fade animation (`TOAST_ATTENTION_PULSE_DURATION ≈ 900ms`) and then settles. Sound is one-shot and does not need dismissal.

## Multi-window behavior

Per-pane state is intentionally focus-independent. A pane in window A can fire its features while window B is focused; the user sees window A's border pulse + glow on the corresponding pane and hears the cue, draws attention there. Toast notifications additionally use `notification_handling = SuppressFromFocusedPane` to suppress popups for the pane the user is already on.
