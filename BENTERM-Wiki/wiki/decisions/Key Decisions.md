---
type: decision
title: "Key Decisions"
created: 2026-04-22
updated: 2026-04-23
status: active
tags:
  - decisions
related:
  - "[[Release Readiness 2026-04-22]]"
  - "[[BENTERM and Hyper Yap Boundary]]"
---

# Key Decisions

## GitHub Releases Are Official

The official BENTERM release channel is the GitHub release with direct platform artifacts and checksums. Package-manager channels are optional follow-ups.

## Cross-Platform Identity

BENTERM should not be framed as Windows-only. Release docs, metadata, and packaging should present Windows, Linux, and macOS as first-class supported platforms when artifacts are available.

## Quiet Notification Defaults

BENTERM suppresses missing-glyph warnings by default to avoid noisy desktop notifications in normal terminal use.

## Wiki In Main, Not Installers Yet

The Obsidian vault should be committed with the repository as project documentation after sanitization. It should not be bundled into end-user installers until there is a stronger reason to ship the vault as runtime/package content.

## Hyper Yap Boundary

BENTERM owns terminal-level defaults such as smart `Ctrl+C`. Hyper Yap owns dictated-block insertion, wrong-window recovery, and guaranteed undo for speech-to-text chunks.
