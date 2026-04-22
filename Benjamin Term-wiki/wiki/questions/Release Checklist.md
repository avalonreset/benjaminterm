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
- Build Windows installer. Done through GitHub Actions.
- Publish GitHub release artifacts. Done for `benjaminterm-v2026.04.21`.
- Test side-by-side install behavior.

Progress:

- Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Local portable zip built: `dist/BenjaminTerm-windows-local-20260421.zip`.
- GitHub Actions release artifacts built successfully for Windows, macOS, and Linux.
- Published prerelease: `https://github.com/avalonreset/BenjaminTerm/releases/tag/benjaminterm-v2026.04.21`.
- Current install gate: wait for user confirmation before replacing or switching active terminals.
