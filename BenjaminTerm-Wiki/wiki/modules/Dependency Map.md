---
type: dependency
title: "Dependency Map"
created: 2026-04-22
updated: 2026-04-22
status: seed
tags:
  - dependencies
  - rust
related:
  - "[[Tech Stack]]"
sources:
  - "[[Repository Source Map]]"
---

# Dependency Map

This page is a seed map of dependency areas rather than a full lockfile analysis.

## Internal Workspace Areas

- Terminal core and parsing: `termwiz`, `wezterm-cell`, `wezterm-escape-parser`.
- GUI/runtime: `wezterm-gui`, `window`, `wezterm-surface`.
- Multiplexing and remote workflows: `mux`, `wezterm-mux-server`, `wezterm-ssh`, `wezterm-client`.
- Configuration: `config`, `wezterm-dynamic`, Lua API crates.
- Packaging and release: `ci/`, `.github/workflows/`, `assets/`.

## Release Risk Notes

- Platform package builds depend on native CI runners or containerized Linux environments.
- macOS signing/notarization depends on external Apple credentials.
- Package-manager publication depends on optional external accounts and tokens.
