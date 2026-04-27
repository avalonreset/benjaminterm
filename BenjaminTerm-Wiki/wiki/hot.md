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

2026-04-27

## Key Recent Facts

- v1.4.5 is the latest tag (released 2026-04-27). Headline changes: cursor-row glow stability + Claude Code OSC 9 bridge. See [[Attention Trigger Lifecycle]] for the full spec.
- The attention trigger is **OSC 9 / OSC 777**. Anything else (idle-detection on `PaneOutput`, BEL → attention) is an anti-pattern. Codex emits OSC 9 natively. Claude Code does NOT - it needs a `Stop` hook that runs `assets/shell-integration/benjaminterm-claude-stop.ps1`. Plain pwsh shells emit OSC 9 from the bundled `assets/shell-integration/benjaminterm.ps1` integration.
- Claude Code hooks are spawned with their stdio redirected to pipes AND detached from the parent conpty. A naive `printf '\033]9;...' > /dev/tty` from a hook lands on an ephemeral console BenjaminTerm never sees. The bundled helper walks the parent process chain, `FreeConsole`s, `AttachConsole(parent_pid)`s, and writes OSC 9 to the now-correct `CONOUT$`. Skip `bash.exe` wrappers (they have their own ephemeral consoles); stop on first non-wrapper success to avoid double-firing.
- Cursor-row glow live-tracks the cursor for ~1s after the ready signal (gives TUI agents time to settle on their real input row), then locks. Cleared in `mark_pane_input` when the user types.
- v1.4.4 made BenjaminTerm auto-discover `~/.benjaminterm.lua` before falling back to `~/.wezterm.lua` (and same in `<config-dir>` and `<exe-dir>` on Windows). Plain launches without `--config-file` now load the user's full config.
- `warn_about_missing_glyphs` default is `false` (matching the documented [[Notification Noise Policy]]).
- BenjaminTerm is a cross-platform WezTerm-based terminal distribution. Release target is GitHub release artifacts for Windows, Linux, and macOS.
- Homebrew, Linuxbrew, Flathub, WinGet, and Gemfury are optional package-manager channels gated by repo variables.
- Smart `Ctrl+C` works: selection copies, no selection interrupts.
- Hyper Yap owns dictated-block undo and wrong-window recovery; BenjaminTerm keeps terminal-level defaults.
- README positioning: BenjaminTerm as a sensory workflow layer for vibe coding, centered on idle glow cues, pane-aware notifications, soft randomized sounds, fresh theme rotation, and fast attention switching.
- Sound cue licensing was traced in the `sound-refresh-soft-cues` worktree: 84 generated WAV files from CC0 Kenney UI Audio and CC0 ObsydianX Interface SFX Pack 1 sources.
- The public README banner lives at `assets/banner.webp`, `21:9` WebP.

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
