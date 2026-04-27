---
type: source
title: "Repository Source Map"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - source-map
  - repository
related:
  - "[[Architecture Overview]]"
---

# Repository Source Map

The repo root is:

```text
<repo-root>
```

## Primary Docs

- `README.md`: public overview, features, install notes.
- `RELEASING_BENTERM.md`: release checklist and publishing policy.
- `RELEASE_NOTES_BENTERM.md`: draft release notes.
- `BENTERM_SUMMARY.txt`: project summary and positioning.

## Release and Packaging

- `ci/generate-workflows.py`: source for generated GitHub workflow YAML.
- `.github/workflows/gen_*_tag.yml`: generated tag release workflows.
- `ci/deploy.sh`: native package build logic.
- `ci/appimage.sh`: AppImage packaging.
- `ci/source-archive.sh`: source archive creation.
- `ci/make-flathub-pr.sh`: optional Flathub PR automation.
- `ci/make-winget-pr.sh`: optional WinGet PR automation.
- `ci/PKGBUILD.template`: Arch package template.
- `ci/wezterm-homebrew-macos.rb.template`: macOS Homebrew cask template.
- `ci/wezterm-linuxbrew.rb.template`: Linuxbrew formula template.

## Branding and Desktop Identity

- `assets/wezterm.desktop`
- `assets/wezterm.appdata.xml`
- `assets/flatpak/`
- `assets/macos/BENTERM.app/`
- `assets/windows/`
- `assets/icon/`

## Behavior and Defaults

- `config/src/config.rs`
- `wezterm-toast-notification/`
- `wezterm-gui/`
- `extras/vibe/`
