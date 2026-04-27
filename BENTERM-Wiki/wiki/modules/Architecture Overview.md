---
type: module
title: "Architecture Overview"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - architecture
  - rust
  - terminal
related:
  - "[[Tech Stack]]"
  - "[[Packaging and CI]]"
  - "[[Terminal Defaults]]"
sources:
  - "[[Repository Source Map]]"
---

# Architecture Overview

BENTERM is a WezTerm-based Rust workspace. The current fork work is concentrated in branding, packaging, release automation, config defaults, notification behavior, and GUI/runtime polish.

## Major Areas

- `wezterm-gui/`: GUI frontend and terminal window behavior.
- `wezterm/`: command-line entrypoint crate.
- `wezterm-mux-server/`: mux server executable.
- `config/`: configuration schema and defaults.
- `wezterm-toast-notification/`: Windows toast notification integration.
- `ci/`: release packaging and generated GitHub workflow logic.
- `assets/`: desktop metadata, macOS bundle template, icons, shell integration, and platform assets.
- `extras/vibe/`: BENTERM vibe config and Linux bootstrap docs.

## Current Release Focus

The important architectural boundary is that crate names still largely follow upstream WezTerm, while package names, executable names, desktop IDs, app bundle IDs, and artifact names use BENTERM branding.
