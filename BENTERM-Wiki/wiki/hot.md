---
type: cache
title: "Hot"
created: 2026-04-22
updated: 2026-04-28
status: active
tags:
  - hot-cache
---

# Hot

## Last Updated

2026-04-28

## Key Recent Facts

- **v2.0.0 is the latest tag** (re-released 2026-04-28). Headline change: full WezTerm visual brand purge. Same engine as v1.4.5, no functional changes - banner / social preview / Linux + macOS + Windows icons / desktop integration files / Flatpak templates all rebranded to BENTERM with the BEN red graphic. See [[v2.0.0 Rebrand Release]] and [[Release Workflow Tax]].
- v2.0.0 release artifacts: `benterm-v2.0.0-setup.exe`, `benterm-windows-v2.0.0.zip`, `benterm-macos-v2.0.0.zip`, `benterm-linux-v2.0.0.tar.gz` plus four `.sha256` files. Live at https://github.com/avalonreset/benterm/releases/tag/v2.0.0.
- The bundled distro Lua config now ships as `benterm.lua` (was `wezterm.lua`). Binary still falls back to `.wezterm.lua` and `.benjaminterm.lua` per [[config search order]].
- Internal namespacing intentionally NOT renamed in v2.0.0: `org.wezfurlong.wezterm` window class, mux socket prefix, `%LOCALAPPDATA%\wezterm` data dir, terminfo entry, Cargo package names. Hyper Yap targets the window class for hotkey detection - rename would break interop. Revisit in v3.
- v1.4.5 brought cursor-row glow stability + Claude Code OSC 9 bridge. See [[Attention Trigger Lifecycle]] for the canonical OSC 9 / OSC 777 spec.
- The attention trigger is **OSC 9 / OSC 777**. Anything else (idle-detection on `PaneOutput`, BEL → attention) is an anti-pattern. Codex emits OSC 9 natively. Claude Code does NOT - it needs a `Stop` hook that runs `assets/shell-integration/benterm-claude-stop.ps1`. Plain pwsh shells emit OSC 9 from the bundled `assets/shell-integration/benterm.ps1` integration.
- v1.4.4 made BENTERM auto-discover `~/.benterm.lua` before falling back to `~/.wezterm.lua` (and same in `<config-dir>` and `<exe-dir>` on Windows).
- `warn_about_missing_glyphs` default is `false` (matches [[Notification Noise Policy]]).
- BENTERM is a cross-platform WezTerm-based terminal distribution. Release target is GitHub release artifacts for Windows, Linux, and macOS via `.github/workflows/benterm-release.yml`.
- Homebrew, Linuxbrew, Flathub, WinGet, and Gemfury are optional package-manager channels gated by repo variables.
- Smart `Ctrl+C` works: selection copies, no selection interrupts.
- Hyper Yap owns dictated-block undo and wrong-window recovery; BENTERM keeps terminal-level defaults.
- README banner lives at `assets/banner.webp`, 1915×821 WebP. GitHub social preview lives at `assets/github-social-preview.jpg`, 1280×640 JPEG. Both regenerated via KIE.ai gpt-image-2 (`scripts/regenerate-banner.py` is the reusable script).
- Windows icon = `assets/windows/terminal.ico` (7 sizes 16-256). macOS icon = `assets/macos/BENTERM.app/Contents/Resources/terminal.icns` (7 sizes 16-1024). Linux icon = `assets/icon/terminal.png` (128px). Source for all three = `assets/icon/BENTERM.jpg` (1024×1024 BEN red graphic). Run `assets/icon/update.sh` to regenerate (needs `magick` + `png2icns`; on Windows we used Pillow as a `png2icns` substitute).
- Sound cue licensing was traced in the `sound-refresh-soft-cues` worktree: 84 generated WAV files from CC0 Kenney UI Audio and CC0 ObsydianX Interface SFX Pack 1 sources.

## Recent Changes

- **2026-04-28**: v2.0.0 force-pushed and re-released with full visual rebrand. WezTerm-named SVG icons removed, BENTERM red graphic propagated to all three platforms' icons. macOS bundle dir renamed `WezTerm.app` → `BENTERM.app`. Linux desktop integration files renamed to `benterm.*` with `com.avalonreset.benterm` ids. Flatpak templates renamed and rewritten. README softened. Bundled config ships as `benterm.lua`. macOS `build.rs` bug caught (hardcoded `WezTerm.app` path) and fixed in `4ccbe04a6`.
- v2.0.0 ship cost: 2 CI runs, ~52 min wall clock from first push to release-live. First run macOS-failed at ~17 min (build.rs); first run Windows was cancelled mid-flight to retag (in retrospect, should have let it finish so the .exe would land in the draft release before retag wiped it). [[Release Workflow Tax]] documents the design issues that made the recovery slower than it should have been.
- v1.4.5 shipped 2026-04-27 (BenjaminTerm-branded artifacts).
- CI packaging was rebranded toward BENTERM artifact names and app IDs.
- A repo-local Obsidian vault was scaffolded at `BENTERM-Wiki`.
- The vault now includes templates, folder indexes, a source map, architecture notes, and a canvas.
- Added public-release sanitization rules and replaced local absolute paths with `<repo-root>`.
- Added native smart `Ctrl+C` behavior: copy selected text, otherwise send interrupt.
- Recorded the [[BENTERM and Hyper Yap Boundary]] decision.
- Added [[Sound Library Licensing]], [[Sound Grab Bag Attention System]], and [[Sound Cue Source Manifest]].

## Active Threads

- ⚠️ Post-install first BENTERM window has UIPI-shaped HyperYap integration failure - documented in [[Post-Install First-Window Quirks]]. Fix is one-line `runasoriginaluser` flag in `ci/windows-installer.iss` `[Run]` section. Land in next installer touch.
- ⚠️ v2.0.0 installer on a machine with the legacy v1.x install lands at `C:\Program Files\BenjaminTerm\` instead of `C:\Program Files\BENTERM\` - cosmetic only (binary IS v2.0.0), fix documented in [[Post-Install First-Window Quirks]] (add `DisableDirPage=auto` + `UsePreviousAppDir=no` to `[Setup]`).
- Verify v2.0.0 install on the maintainer's Windows machine via the official `benterm-v2.0.0-setup.exe` (font system-wide, BEN icon in Apps & Features, Start Menu shortcut, registry "Open BENTERM here" entries).
- Optimize `benterm-release.yml`: wire `sccache` (drops subsequent builds from ~25 min to ~10), make `prepare-release` idempotent (skip-if-exists rather than delete-then-create), split macOS universal job into two parallel arch-specific jobs. See [[Release Workflow Tax]].
- Fix legacy `ci/deploy.sh` and `ci/appimage.sh` so the per-distro `*_continuous.yml` workflows turn green again (still reference `assets/wezterm.appdata.xml`, `assets/wezterm.desktop`, `WezTerm.app` etc. - all renamed in v2.0.0). Doesn't ship anything but the badge is red.
- Decide whether macOS release should be signed/notarized before public announcement (currently `MACOS_TEAM_ID` secret isn't set so the signing block in `package-benterm-macos.sh` is a no-op).
- Keep optional package-manager publishing disabled until target repos/accounts are ready.
- Use [[Wiki Maintenance Flow]] as the default process for future vault edits.
- Run [[Public Release Sanitization]] before committing or publishing the vault.
- Before advertising sound cues in a release, merge the sound refresh worktree changes and confirm sound source notes ship with artifacts.
- Do not attribute the rejected `kenney-interface` prototype set unless it appears in the artifact being shipped.
- Keep banner generation credentials and local candidate paths out of durable vault notes.
