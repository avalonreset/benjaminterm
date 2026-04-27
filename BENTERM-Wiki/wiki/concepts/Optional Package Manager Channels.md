---
type: concept
title: "Optional Package Manager Channels"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - package-managers
  - release
related:
  - "[[Packaging and CI]]"
  - "[[Release Flow]]"
sources:
  - "[[Project Documentation]]"
---

# Optional Package Manager Channels

Homebrew, Linuxbrew, Flathub, WinGet, and Gemfury are optional convenience channels. They are not required for an official BENTERM release.

## Control Flags

- `PUBLISH_HOMEBREW`
- `PUBLISH_LINUXBREW`
- `PUBLISH_FLATHUB`
- `PUBLISH_WINGET`
- `PUBLISH_GEMFURY`

Each channel only runs when its repository variable is set to `true`.

## Release Principle

GitHub release artifacts should upload before optional ecosystem publishing runs, so missing external accounts do not block the release.
