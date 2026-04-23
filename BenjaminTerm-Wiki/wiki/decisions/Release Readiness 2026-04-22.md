---
type: decision
title: "Release Readiness 2026-04-22"
created: 2026-04-22
updated: 2026-04-23
status: release-candidate
tags:
  - release
  - ci
  - validation
related:
  - "[[Packaging and CI]]"
  - "[[Release Flow]]"
  - "[[Release Candidate Definition]]"
  - "[[BenjaminTerm and Hyper Yap Boundary]]"
---

# Release Readiness 2026-04-22

BenjaminTerm is ready to push as a release candidate, but it is not an official three-platform release until GitHub Actions successfully builds and uploads Windows, Linux, and macOS artifacts.

## Passed Checks

- Release shell scripts parse with `bash -n`.
- Generated GitHub workflow YAML parses.
- Python release helpers compile.
- `cargo check -p wezterm-toast-notification` passes.
- `cargo check -p wezterm-gui --quiet` passes with existing warnings only.
- Smart `Ctrl+C` now has a native action and default key binding.

## Release Blockers

- None known in local validation.

## Artifact Status

- Local Windows artifacts exist from the current branch state, but generated artifacts should be considered disposable until the final release commit is tagged.
- Official Windows, Linux, and macOS release status depends on the GitHub tag workflows building and uploading artifacts successfully.
- Existing GitHub release artifacts are not affected by local working-tree changes until a new release is published.

## External Conditions

- macOS signing/notarization requires Apple Developer credentials if a polished public macOS experience is required.
- Package-manager submissions are optional and are gated by explicit repository variables.
- Final artifact readiness depends on tag workflows running green.
