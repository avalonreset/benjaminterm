---
type: meta
title: "Rebuild Release State"
created: 2026-04-22
updated: 2026-04-22
tags:
  - release
  - benjaminterm
status: active
related:
  - "[[Release Checklist]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "partial"
---

# Rebuild Release State

Current rebuild status:

- Core config defaults are working in preview.
- Theme shuffle-bag works in preview.
- 0xProto is bundled in the source tree.
- Windows toast click-to-focus compiles and now uses the BenjaminTerm AppUserModelID `com.avalonreset.benjaminterm`.
- Attention pulse, tab marker, per-pane sound prototype, and input-cleared toast reminders compile.
- Source-built preview launched from `target-sound-prototype`.
- Windows portable packaging exists via `ci/package-benjaminterm-windows.ps1`.
- Local release build succeeded and produced `dist/BenjaminTerm-windows-local-20260421.zip`.
- Installer script is rebranded to Benjamin Term, uses a unique Inno Setup GUID, installs bundled `wezterm.lua`, fonts, and sounds, and avoids adding `wezterm.exe` to PATH.
- GitHub Actions release workflow exists for tag-driven `benjaminterm-v*` releases across Windows, macOS, and Linux.

Next release work:

- Build the Windows installer on a machine with Inno Setup or through GitHub Actions.
- Push a `benjaminterm-v*` tag to publish Windows/macOS/Linux artifacts.
- Treat macOS and Linux artifacts as best-effort until manually tested on those platforms.
