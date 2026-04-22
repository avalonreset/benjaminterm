---
type: question
title: "Release Checklist"
created: 2026-04-22
updated: 2026-04-22
tags:
  - release
status: open
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
- Build release artifact.
- Build Windows installer.
- Publish GitHub release artifacts.
- Test side-by-side install behavior.

Progress:

- Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Local portable zip built: `dist/BenjaminTerm-windows-local-20260421.zip`.
- Inno Setup is not installed on the current Windows machine, so the local `.exe` installer still needs either Inno Setup locally or the GitHub Actions release workflow.
