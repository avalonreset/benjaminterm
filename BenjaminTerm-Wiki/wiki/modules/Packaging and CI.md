---
type: module
title: "Packaging and CI"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - ci
  - packaging
  - release
related:
  - "[[Release Flow]]"
  - "[[Optional Package Manager Channels]]"
  - "[[Cross-Platform Release Strategy]]"
---

# Packaging and CI

Packaging and CI now target BenjaminTerm-branded release artifacts.

## Required GitHub Release Artifacts

- Windows: `BenjaminTerm-*.zip`, `BenjaminTerm-*.exe`, checksums.
- macOS: `BenjaminTerm-macos-*.zip`, checksums.
- Linux: `benjaminterm-*.deb`, `benjaminterm-*.rpm`, `benjaminterm-*.tar.xz`, source archive, AppImage, checksums.

## Optional Channels

These channels are skipped unless their repository variable is set to `true`:

- `PUBLISH_HOMEBREW`
- `PUBLISH_LINUXBREW`
- `PUBLISH_FLATHUB`
- `PUBLISH_WINGET`
- `PUBLISH_GEMFURY`

The GitHub release upload runs before optional package-manager steps.

## Important Files

- `ci/generate-workflows.py`
- `ci/deploy.sh`
- `ci/appimage.sh`
- `ci/source-archive.sh`
- `ci/make-flathub-pr.sh`
- `ci/make-winget-pr.sh`
- `.github/workflows/gen_*_tag.yml`
