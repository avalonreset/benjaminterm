# Changelog

## Unreleased

### Changed

- Nothing yet.

## v1.4.5

Cursor-row glow stability + Claude Code attention bridge.

### Added

- Bundled `assets/shell-integration/benjaminterm-claude-stop.ps1` — a Claude Code Stop hook helper that walks the parent process chain, attaches to the conpty BenjaminTerm reads, and writes OSC 9 directly to it. Lets Claude Code emit the agent ready signal the same way Codex does natively, so the per-pane attention features fire when Claude finishes a turn. Wire it from `~/.claude/settings.json` (recipe in the wiki).
- New wiki page `BenjaminTerm-Wiki/wiki/concepts/Attention Trigger Lifecycle.md` documenting the OSC 9 trigger, the per-shell integration patterns, and the anti-patterns to never re-introduce (idle-detection, BEL→attention).

### Changed

- `assets/shell-integration/benjaminterm.ps1` now emits OSC 9 (the spec's agent ready signal per Mandatory Requirements M1) instead of plain BEL, matching what Codex emits natively.
- Missing-glyph warning toast (`warn_about_missing_glyphs`) now defaults to `false`, honoring the documented Notification Noise Policy. Common emoji codepoints in agent output (Claude Code, Codex) no longer fire the upstream warning.

### Fixed

- Cursor-row idle glow no longer drifts mid-render as TUI agents (Claude Code, Codex) move the cursor around their UI. The renderer now live-tracks the cursor for ~1s after the ready signal — enough for the agent's final repaint to land the cursor on its real input row — then locks the glow to that row until the founder types.

## v1.4.4

Resilient first-launch release.

### Added

- Auto-discovers `~/.benjaminterm.lua` (and `<config-dir>/benjaminterm.lua`, and `<exe-dir>/benjaminterm.lua` on Windows) before falling back to the wezterm-named config files. Users with a `.benjaminterm.lua` no longer need to launch with `--config-file` to load their config.
- Bundled pwsh shell-integration script at `assets/shell-integration/benjaminterm.ps1`. Sourcing it from `$PROFILE` enables the attention pulse, cursor-row glow, and soft-cue sound on every prompt without any user-side per-pane wiring.

### Fixed

- Plain BenjaminTerm launches (no `--config-file`, no launcher) now find the user's full daily config automatically. Previously the binary only looked for `.wezterm.lua`, leaving users with `.benjaminterm.lua` silently running on the wezterm-named fallback (or no config at all).

## v1.4.3

Clickable path and paste recovery release.

### Added

- Documented benjaminterm's built-in clickable local path handling in the README.
- Added paste undo and redo to the bundled benjaminterm config.
- `Ctrl+Z` now undoes a recent recorded paste when available and passes through normally otherwise.
- `Ctrl+Shift+Z` redoes the most recent paste undo.

### Changed

- Restricted the release workflow to tag and manual release events so normal `main` pushes do not produce noisy failed release runs.
- Disabled Dependabot branch noise for this release-focused repository.

## v1.4.2

Clickable local path and lowercase naming release.

### Added

- Added built-in hyperlink rules for local terminal output paths in the shipped benjaminterm config.
- Recognizes Windows absolute paths, quoted paths, UNC paths, relative paths, home-relative paths, and common artifact filenames.
- Handles common `file:line` and `file:line:column` suffixes before opening the path.

### Changed

- Updated README and install docs to point to `v1.4.2` artifacts.
- Lowercased the public brand and release artifact names to `benjaminterm`.
- Lowercased Windows portable package executable names, installer app naming, macOS app bundle naming, and Linux tarball executable names.

## v1.4.1

Branding and distribution finalization.

### Changed

- Published Windows installer, Windows portable zip, macOS app zip, Linux tarball, and SHA256 checksums on GitHub Releases.
- Standardized visible app naming as `benjaminterm` across installer, shortcuts, shell labels, and macOS app directory naming.
- Documented shipped sound cue attribution for Kenney UI Audio and ObsydianX Interface SFX Pack 1.
- Confirmed the old `kenney-interface` prototype sound set is not included in the shipped artifacts.
- Declared GitHub Releases as the official distribution channel for all supported platforms.

## v1.4.0

Cross-platform release baseline.

### Added

- Windows installer and portable zip.
- macOS app zip.
- Linux tarball.
- Bundled 0xProto font.
- Bundled `benjaminterm-soft-cues` sound pack.
- Visual attention pulse, per-pane sound identity, and Windows toast click-to-focus.

## v1.3.1

Release infrastructure update.

### Changed

- Pinned repository Node.js version to `25.9.0`.
- Added `.node-version` for local development shells.
- Updated the benjaminterm release workflow to install and verify Node.js `25.9.0` on Windows, macOS, Linux, and release jobs.
- Removed artifact upload and download actions from the release pipeline so release assets are uploaded directly with `gh`.
- Updated official GitHub Actions steps to newer major versions.
- Replaced the third-party release publisher action with direct `gh release` commands.

## v1.3.0

Fresh benjaminterm rebuild release.

### Added

- Windows installer and portable Windows zip.
- macOS and Linux release artifacts.
- benjaminterm release workflow for tags matching `v[0-9]*`.
- Bundled benjaminterm config loaded as `wezterm.lua` in release packages.
- Bundled 0xProto font.
- Bundled Kenney CC0 interface sounds.
- Theme shuffle-bag defaults.
- Per-pane sound grab bag for agent completion cues.
- Visual attention pulse for ready panes.
- Background tab marker for ready panes.
- Windows reminder toasts with click-to-focus.
- Input-cleared toast cleanup.
- Restored README banner, screenshot, BEN logo assets, and social preview.
- README positioning for HyperYap as the sister project and full workstation layer.

### Changed

- Rebuilt from a fresh WezTerm baseline.
- Removed every bundled font except 0xProto.
- Changed the default internal font fallback from JetBrains Mono to 0xProto.
- Removed JetBrains Mono, Fira Code, Roboto, Noto Color Emoji, and Symbols Nerd Font Mono from the benjaminterm source bundle.
- Removed old bundled font license files that no longer apply.
- Updated README and license language so the public GitHub page accurately says benjaminterm bundles 0xProto only.
- Expanded README philosophy for the 0xProto decision and the attention cue system.
- Removed the old smart paste and image paste direction from benjaminterm scope.
- Delegated speech-to-text and clipboard workflow to HyperYap.
- Rebranded Windows installer identity to benjaminterm.
- Added a distinct Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Added a distinct Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Disabled installer PATH edits to avoid shadowing vanilla WezTerm.
- Cleaned stale upstream support and funding links from the benjaminterm GitHub surface.

### Historical Caveats

- This release was superseded by `v1.4.1`, which publishes official Windows, macOS, and Linux GitHub release artifacts.
- Windows remains the platform with the strongest notification integration because it includes toast click-to-focus.
