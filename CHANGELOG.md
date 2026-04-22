# Changelog

## v1.3.1

Release infrastructure update.

### Changed

- Pinned repository Node.js version to `25.9.0`.
- Added `.node-version` for local development shells.
- Updated the BenjaminTerm release workflow to install and verify Node.js `25.9.0` on Windows, macOS, Linux, and the publisher job.
- Updated official GitHub Actions steps to newer major versions.
- Replaced the third-party release publisher action with direct `gh release create` usage.

## v1.3.0

Fresh BenjaminTerm rebuild release.

### Added

- Windows installer and portable Windows zip.
- Best-effort macOS and Linux release artifacts.
- BenjaminTerm release workflow for tags matching `v[0-9]*`.
- Bundled BenjaminTerm config loaded as `wezterm.lua` in release packages.
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
- Removed JetBrains Mono, Fira Code, Roboto, Noto Color Emoji, and Symbols Nerd Font Mono from the BenjaminTerm source bundle.
- Removed old bundled font license files that no longer apply.
- Updated README and license language so the public GitHub page accurately says BenjaminTerm bundles 0xProto only.
- Expanded README philosophy for the 0xProto decision and the attention cue system.
- Removed the old smart paste and image paste direction from BenjaminTerm scope.
- Delegated speech-to-text and clipboard workflow to HyperYap.
- Rebranded Windows installer identity to Benjamin Term.
- Added a distinct Windows AppUserModelID: `com.avalonreset.benjaminterm`.
- Added a distinct Windows installer GUID: `E79835B5-C418-4C79-BD62-3A18E94B22C3`.
- Disabled installer PATH edits to avoid shadowing vanilla WezTerm.
- Cleaned stale upstream support and funding links from the BenjaminTerm GitHub surface.

### Known Caveats

- Windows is the primary supported platform.
- macOS and Linux artifacts are best-effort builds and need manual testing.
- macOS and Linux notification behavior is not yet equivalent to the Windows toast implementation.
