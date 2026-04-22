---
type: question
title: "Release Checklist"
created: 2026-04-22
updated: 2026-04-22
tags:
  - release
status: active
related:
  - "[[Rebuild Release State]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Release Checklist

- Launch source-built preview.
- Verify theme shuffle-bag.
- Verify bundled 0xProto loading.
- Verify Windows toast notification.
- Verify toast click-to-focus.
- Verify attention pulse.
- Assign distinct Windows AppUserModelID.
- Rebrand Windows installer identity.
- Package bundled config as `wezterm.lua`.
- Package bundled fonts.
- Package bundled sound grab bag.
- Confirm license notices include 0xProto OFL.
- Build release artifact. Done for Windows/macOS/Linux.
- Build Windows installer.
- Publish GitHub release artifacts for `v1.4.0`.
- Replace the current local BenjaminTerm install with `v1.4.0`.

Progress:

- Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Local portable zip built: `dist/BenjaminTerm-windows-local-20260421.zip`.
- GitHub Actions release artifacts built successfully for Windows, macOS, and Linux.
- Published prerelease: `https://github.com/avalonreset/BenjaminTerm/releases/tag/benjaminterm-v2026.04.21`.
- Current install gate: user asked to replace the active local BenjaminTerm install with the new release.
- Corrective font packaging pass: 0xProto is now the only bundled font, and local package inspection confirms only `0xProto-Bold.ttf`, `0xProto-Italic.ttf`, `0xProto-Regular.ttf`, and `LICENSE_0XPROTO.txt` ship in `fonts/`.
- Sound-refresh package: `dist/BenjaminTerm-windows-v2026.04.22-soft-cues-curated.zip`.
- Release package: `dist/BenjaminTerm-windows-v1.4.0.zip`.
- Sound-refresh cue count: 84 WAV files, 51 Kenney UI Audio plus 33 ObsydianX Interface SFX Pack 1.
- Sound-refresh normalization: per-file peak normalized around `-14 dB`; packaged measurement quietest `-14.0 dB`, loudest `-13.9 dB`.
- Local Inno Setup compiler: `C:/Users/rccol/AppData/Local/Programs/Inno Setup 6/ISCC.exe`.
- `v1.4.0` branding gate: verify `BenjaminTerm-gui.exe` product metadata and icon before installing.
