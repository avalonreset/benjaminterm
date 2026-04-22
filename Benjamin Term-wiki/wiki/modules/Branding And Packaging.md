---
type: module
title: "Branding And Packaging"
created: 2026-04-22
updated: 2026-04-22
tags:
  - branding
  - packaging
status: active
related:
  - "[[Rebuild Release State]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Branding And Packaging

Branding and packaging must make BenjaminTerm install side-by-side with upstream WezTerm and the old BenjaminTerm until final replacement.

Current implementation:

- Windows installer is named Benjamin Term and emits `BenjaminTerm-*.exe`.
- Windows Start Menu/Desktop shortcuts use AppUserModelID `com.avalonreset.benjaminterm`.
- Windows installer uses unique Inno Setup AppId `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Installer and portable zip include `wezterm.lua`, `fonts/`, and `sounds/kenney-interface/`.
- Installer no longer edits PATH, which avoids shadowing vanilla `wezterm.exe`.
- Local portable zip packaging is handled by `ci/package-benjaminterm-windows.ps1`.
- Tag-driven GitHub release workflow is handled by `.github/workflows/benjaminterm-release.yml`.

Remaining work:

- Build and verify the Inno Setup installer.
- Decide whether to rename shipped executable files or keep upstream binary names inside the BenjaminTerm install directory.
- Fully rebrand macOS app bundle metadata.
- Fully rebrand Linux desktop/appdata/package metadata.
