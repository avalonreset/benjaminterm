---
type: flow
title: "Release Flow"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - release
  - workflow
related:
  - "[[Packaging and CI]]"
  - "[[Release Readiness 2026-04-22]]"
  - "[[Cross-Platform Release Strategy]]"
  - "[[Optional Package Manager Channels]]"
---

# Release Flow

## Normal Release

1. Push release-candidate branch.
2. Open and merge PR.
3. Tag the release.
4. Let tag workflows build platform artifacts.
5. Confirm the GitHub release includes checksums and platform artifacts.
6. Publish release notes with platform install status.

## Optional Package Managers

Only enable package-manager variables after the corresponding account/repository/token is configured:

- Homebrew: `PUBLISH_HOMEBREW=true`
- Linuxbrew: `PUBLISH_LINUXBREW=true`
- Flathub: `PUBLISH_FLATHUB=true`
- WinGet: `PUBLISH_WINGET=true`
- Gemfury: `PUBLISH_GEMFURY=true`

Leaving these unset is valid and does not block the GitHub release.
