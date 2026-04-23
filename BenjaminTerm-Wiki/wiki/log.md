---
type: log
title: "Wiki Log"
created: 2026-04-22
updated: 2026-04-23
status: active
tags:
  - log
---

# Wiki Log

## 2026-04-23 - Banner Workflow Added And Sanitized

- Added [[README Banner Asset Workflow]] to document the public README banner path, `21:9` ratio, WebP delivery format, and local candidate promotion flow.
- Updated [[overview]], [[hot]], [[index]], and [[Concepts Index]] so the vault reflects the current public README surface.
- Removed remaining machine-specific path references from sound and validation notes.
- Confirmed local Obsidian workspace state remains ignored rather than tracked in the public vault.

## 2026-04-23 - Sound Cue Licensing Documented

- Traced BenjaminTerm sound cues to the `sound-refresh-soft-cues` worktree.
- Added [[Sound Library Licensing]], [[Sound Grab Bag Attention System]], and [[Sound Cue Source Manifest]].
- Updated release hygiene notes so bundled sound cues keep their README, CC0 license note, and per-file source manifest.
- Verified `BenjaminTerm-windows-v1.4.0.zip`, `BenjaminTerm-windows-v1.4.1.zip`, and a local installed BenjaminTerm build use `benjaminterm-soft-cues` and do not include the old `kenney-interface` prototype directory.

## 2026-04-23 - README Vision Updated

- Updated README positioning around vibe coding, idle glow cues, pane-aware notifications, soft randomized attention sounds, fresh theme rotation, and attention switching.
- Mirrored the product vision in [[overview]] and [[BenjaminTerm]].
- Later sound tracing found the implementation and generated asset pack in the `sound-refresh-soft-cues` worktree.

## 2026-04-23 - Hyper Yap Boundary Recorded

- Added [[BenjaminTerm and Hyper Yap Boundary]].
- Captured decision that BenjaminTerm owns smart terminal defaults while Hyper Yap owns dictated-block undo and wrong-window recovery.
- Clarified release readiness: local changes do not affect existing GitHub artifacts until a new tag/release is published.

## 2026-04-22 - Smart Ctrl+C Added

- Added native `CopySelectionOrSendKey` behavior for smart `Ctrl+C`.
- Documented [[Smart Copy Interrupt]] and updated release readiness notes.
- Validation: `cargo check -p config` and `cargo check -p wezterm-gui --quiet` pass with existing warnings.

## 2026-04-22 - Sanitization Policy Added

- Added [[Public Release Sanitization]].
- Replaced local absolute paths in seed notes with `<repo-root>`.
- Added vault `.gitignore` rules for local Obsidian state, private raw material, private attachments, and secret-like files.
- Removed `.obsidian/workspace.json` from the scaffold so local workspace state is not part of the public wiki.

## 2026-04-22 - Graph Connections Expanded

- Added `START HERE.md`.
- Added concept notes for cross-platform release strategy, optional package-manager channels, notification noise policy, and release-candidate definition.
- Added [[Wiki Maintenance Flow]] and expanded canvas connectivity.
- Added Obsidian bookmarks for start page, hot cache, dashboard, and visual map.

## 2026-04-22 - Vault Operations Layer Added

- Added note templates for decisions, modules, questions, release checks, and sources.
- Added folder indexes, architecture notes, dependency map, repository source map, operating guide, lint report, and main canvas.
- Updated index and hot cache.

## 2026-04-22 - Vault Created

- Created repo-local Obsidian vault at `BenjaminTerm-Wiki`.
- Seeded repository-mode pages for release readiness, packaging, terminal defaults, release flow, decisions, and open questions.
- Referenced repository docs in `.raw/.manifest.json`.
