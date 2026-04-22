---
type: flow
title: "Preview Testing Flow"
created: 2026-04-22
updated: 2026-04-22
tags:
  - testing
  - preview
status: active
related:
  - "[[Rebuild Release State]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Preview Testing Flow

Preview goals:

- Keep current BenjaminTerm and upstream WezTerm untouched.
- Do not use installed upstream WezTerm for BenjaminTerm feature review.
- Treat installed `C:\Program Files\BenjaminTerm\` windows as release/install-state windows, not as proof of the source worktree state.
- Test source-level behavior through a source-built BenjaminTerm preview executable. Do not launch raw `wezterm-gui.exe` for BenjaminTerm feature review.
- Current temporary Windows preview path is `target/debug/BenjaminTerm-gui.exe`, copied from the built GUI after `wezterm-gui/build.rs` embeds BenjaminTerm resource metadata and icon.
- Use `scripts/preview/launch-benjaminterm-glow-preview.ps1` for idle glow review. It launches the source preview and emits OSC 777 from a script file, avoiding PowerShell inline quoting failures.

Toast pulse testing requires a source-built preview because copied old binaries do not contain the new pulse code.

Idle glow testing cannot rely on a plain shell sitting at a prompt. The current design arms from the agent-ready attention path, so preview testing should launch PowerShell and emit OSC 777 after startup, or use a future debug-only action that triggers the same path.

