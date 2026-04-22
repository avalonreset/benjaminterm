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
validation: "release-published"
---

# Rebuild Release State

Current rebuild status:

- Core config defaults are working in preview.
- Theme shuffle-bag works in preview.
- 0xProto is the only bundled font in the source tree.
- Windows toast click-to-focus compiles and now uses the BenjaminTerm AppUserModelID `com.avalonreset.benjaminterm`.
- Attention pulse, tab marker, per-pane sound prototype, and input-cleared toast reminders compile.
- Source-built preview launched from `target-sound-prototype`.
- Windows portable packaging exists via `ci/package-benjaminterm-windows.ps1`.
- Local release build succeeded and produced `dist/BenjaminTerm-windows-local-20260421.zip`.
- Installer script is rebranded to Benjamin Term, uses a unique Inno Setup GUID, installs bundled `wezterm.lua`, fonts, and sounds, and avoids adding `wezterm.exe` to PATH.
- GitHub Actions release workflow exists for tag-driven `benjaminterm-v*` releases across Windows, macOS, and Linux.
- Release tag `benjaminterm-v2026.04.21` is published on GitHub.
- Corrective release work for `benjaminterm-v2026.04.22` removes the old upstream font bundle and keeps only 0xProto plus its license.
- GitHub Actions successfully built Windows, macOS, and Linux artifacts.
- The Windows installer, Windows portable zip, macOS zip, Linux tarball, and SHA256 files are attached to the prerelease.

Next release work:

- Publish and install the corrected Windows release after the user confirms terminal cutover timing.
- Treat macOS and Linux artifacts as best-effort until manually tested on those platforms.
- Improve macOS/Linux notification behavior as follow-up work after the Windows release is stable.
