---
type: cache
title: "Hot"
created: 2026-04-22
updated: 2026-04-23
status: active
tags:
  - hot-cache
---

# Hot

## Last Updated

2026-04-23

## Key Recent Facts

- BenjaminTerm is being prepared as a cross-platform WezTerm-based terminal distribution.
- The current release target is GitHub release artifacts for Windows, Linux, and macOS.
- Homebrew, Linuxbrew, Flathub, WinGet, and Gemfury are optional package-manager channels gated by repo variables.
- Missing-glyph notifications are suppressed by default for BenjaminTerm.
- Smart `Ctrl+C` was tested in a spawned dev window and works: selection copies, no selection interrupts.
- Hyper Yap should own dictated-block undo and wrong-window recovery; BenjaminTerm should keep terminal-level defaults.
- README positioning now explains BenjaminTerm as a sensory workflow layer for vibe coding, centered on idle glow cues, pane-aware notifications, soft randomized sounds, fresh theme rotation, and fast attention switching.
- Sound cue licensing was traced in the `sound-refresh-soft-cues` worktree: 84 generated WAV files from CC0 Kenney UI Audio and CC0 ObsydianX Interface SFX Pack 1 sources.
- Latest checked packages `BenjaminTerm-windows-v1.4.0.zip` and `BenjaminTerm-windows-v1.4.1.zip` each contain 84 `benjaminterm-soft-cues` WAVs and 0 old `kenney-interface` entries.
- The public README banner now lives at `assets/banner.webp` and should remain a `21:9` WebP asset promoted from reviewed local candidates.

## Recent Changes

- CI packaging was rebranded toward BenjaminTerm artifact names and app IDs.
- `RELEASING_BENJAMINTERM.md`, `RELEASE_NOTES_BENJAMINTERM.md`, and `README.md` were updated for Mac/Linux release status.
- A repo-local Obsidian vault was scaffolded at `BenjaminTerm-Wiki`.
- The vault now includes templates, folder indexes, a source map, architecture notes, and a canvas.
- Added a top-level start page plus concept notes for release strategy, package-manager channels, notification policy, and release-candidate criteria.
- Added public-release sanitization rules and replaced local absolute paths with `<repo-root>`.
- Added native smart `Ctrl+C` behavior: copy selected text, otherwise send interrupt.
- Recorded the [[BenjaminTerm and Hyper Yap Boundary]] decision.
- Updated README vision language and mirrored it in [[overview]] and [[BenjaminTerm]].
- Added [[Sound Library Licensing]], [[Sound Grab Bag Attention System]], and [[Sound Cue Source Manifest]].
- Added [[README Banner Asset Workflow]] and aligned the vault with the live public README banner asset.
- Removed remaining machine-specific path references from wiki notes so the vault stays publishable with the repo.

## Active Threads

- Confirm tag workflows build and upload all platform artifacts.
- Decide whether macOS release should be signed/notarized before public announcement.
- Keep optional package-manager publishing disabled until target repos/accounts are ready.
- Use [[Wiki Maintenance Flow]] as the default process for future vault edits.
- Run [[Public Release Sanitization]] before committing or publishing the vault.
- Publish a new release only when ready to ship the smart `Ctrl+C` fix; existing published artifacts are unchanged by local work.
- Before advertising sound cues in a release, merge the sound refresh worktree changes and confirm sound source notes ship with artifacts.
- Do not attribute the rejected `kenney-interface` prototype set unless it appears in the artifact being shipped.
- Keep banner generation credentials and local candidate paths out of durable vault notes.
