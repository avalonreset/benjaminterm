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
  - "[[Soft Cue Pack Refresh]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Branding And Packaging

Branding and packaging must make BENTERM install side-by-side with upstream WezTerm and the old BENTERM until final replacement.

Current implementation:

- Windows installer is named BENTERM and emits `BENTERM-*.exe`.
- Windows Start Menu/Desktop shortcuts use AppUserModelID `com.avalonreset.benterm`.
- Windows installer uses unique Inno Setup AppId `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Installer and portable zip include `wezterm.lua`, `fonts/`, and `sounds/benterm-soft-cues/`.
- Current release portable package is `dist/BENTERM-windows-v1.4.0.zip`.
- Local installer rebuild uses `C:/Users/rccol/AppData/Local/Programs/Inno Setup 6/ISCC.exe`.
- Installer no longer edits PATH, which avoids shadowing vanilla `wezterm.exe`.
- Local portable zip packaging is handled by `ci/package-benterm-windows.ps1`.
- Tag-driven GitHub release workflow is handled by `.github/workflows/benterm-release.yml`.
- Source GUI Windows resource metadata now uses BENTERM product/file description fields instead of WezTerm fields.
- Source previews may copy the built GUI to `target/debug/BENTERM-gui.exe`, but release/install validation must use the packaged `BENTERM-gui.exe`.

Remaining work:

- Build and verify the Inno Setup installer.
- Replace the current local `C:\Program Files\BENTERM\` install with the `v1.4.0` package/installer.
- Make `BENTERM-gui.exe` a first-class build artifact instead of a post-build copy.
- Decide whether to keep any upstream executable names internally for compatibility, but stop using them in user-facing release/test instructions.
- Fully rebrand macOS app bundle metadata.
- Fully rebrand Linux desktop/appdata/package metadata.
