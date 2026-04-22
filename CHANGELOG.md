# Changelog

## benjaminterm-v2026.04.22

Corrective preview release for the BenjaminTerm rebuild.

### Changed

- Removed every bundled font except 0xProto.
- Changed the default internal font fallback from JetBrains Mono to 0xProto.
- Removed JetBrains Mono, Fira Code, Roboto, Noto Color Emoji, and Symbols Nerd Font Mono from the BenjaminTerm source bundle.
- Removed old bundled font license files that no longer apply.
- Updated README and license language so the public GitHub page accurately says BenjaminTerm bundles 0xProto only.
- Expanded README philosophy for the 0xProto decision and the attention cue system.

## benjaminterm-v2026.04.21

Preview release for the fresh BenjaminTerm rebuild.

### Added

- Windows installer and portable Windows zip.
- Best-effort macOS and Linux release artifacts.
- BenjaminTerm release workflow for tags matching `benjaminterm-v*`.
- Bundled BenjaminTerm config loaded as `wezterm.lua` in release packages.
- Bundled 0xProto font.
- Bundled Kenney CC0 interface sounds.
- Theme shuffle-bag defaults.
- Per-pane sound grab bag for agent completion cues.
- Visual attention pulse for ready panes.
- Background tab marker for ready panes.
- Windows reminder toasts with click-to-focus.
- Input-cleared toast cleanup.
- README documentation for the font philosophy and attention cue strategy.

### Changed

- Rebuilt from a fresh WezTerm baseline.
- Removed the old smart paste and image paste direction from BenjaminTerm scope.
- Delegated speech-to-text and clipboard workflow to HyperYap.
- Rebranded Windows installer identity to Benjamin Term.
- Added a distinct Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Added a distinct Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Disabled installer PATH edits to avoid shadowing vanilla WezTerm.
- Cleaned stale upstream support and funding links from the BenjaminTerm GitHub surface.

### Known Caveats

- Windows is the supported platform for this preview.
- macOS and Linux artifacts are best-effort builds and need manual testing.
- macOS and Linux notification behavior is not yet equivalent to the Windows toast implementation.
