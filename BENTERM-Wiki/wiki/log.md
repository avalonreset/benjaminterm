---
type: log
title: "Wiki Log"
created: 2026-04-22
updated: 2026-04-28
status: active
tags:
  - log
---

# Wiki Log

## 2026-04-28 (later) - Post-Install Quirks Discovered + Documented

After the v2.0.0 install on the maintainer's Windows box, two install-time quirks surfaced. Neither blocks the v2.0.0 release (already shipped), but both deserve fixing in the next `ci/windows-installer.iss` touch.

**Quirk 1: HyperYap completely dead in the first BENTERM window after install.** Subsequent windows work fine. Leading hypothesis: UIPI token mismatch. Inno Setup's `[Run]` step inherits the installer's elevated token unless `runasoriginaluser` is set, our `.iss` does not set it, so the post-install BENTERM is High IL. HyperYap (Medium IL, normal user launch) cannot send keystrokes / paste events into a High IL window per Windows UIPI rules. Direct evidence eluded the diagnostic - the original elevated process was already closed by the time we checked, all currently-running BENTERMs are Medium IL. Symptom + .iss code path + UIPI behavior all line up. Fix: add `runasoriginaluser` to the `[Run]` Flags. See [[Post-Install First-Window Quirks]].

**Quirk 2: v2.0.0 installer lands at `C:\Program Files\BenjaminTerm\` not `C:\Program Files\BENTERM\` on machines that had a legacy BenjaminTerm install.** Registry shows two `_is1` entries with different AppIds both pointing at the same legacy folder. Binary IS the correct v2.0.0 build (file version `20260427-233347-4ccbe04a` matches the release commit), only the folder name is off. Fix: add `DisableDirPage=auto` + `UsePreviousAppDir=no` to `[Setup]`. See [[Post-Install First-Window Quirks]].

No re-release needed for either - they hit once on first install and never again. Punch-listed for the next installer-touching commit alongside the [[Release Workflow Tax]] items.

## 2026-04-28 - v2.0.0 Re-Released: WezTerm Visual Brand Purge + Banner / Icon Regen

Force-pushed `v2.0.0` to a new commit (`4ccbe04a6`) replacing the original v2.0.0 release. The rebrand was a pure-visual pass, not a functional change. Internal namespacing (`org.wezfurlong.wezterm` window class, mux socket prefix, `%LOCALAPPDATA%\wezterm` data dir, terminfo entry, Cargo package names) is intentionally unchanged because tools like Hyper Yap key off the window class and renaming would break interop. Layer to revisit in v3.

**Shipped artifacts** (https://github.com/avalonreset/benterm/releases/tag/v2.0.0):
- `benterm-v2.0.0-setup.exe` (Windows Inno Setup installer, 42.7 MB)
- `benterm-windows-v2.0.0.zip` (Windows portable, 68 MB)
- `benterm-macos-v2.0.0.zip` (macOS universal x86_64 + arm64, 96 MB)
- `benterm-linux-v2.0.0.tar.gz` (Linux portable, 49.9 MB)
- `.sha256` next to each.

**Visual rebrand work** (commit `7c93e6b27`):
- New `assets/banner.webp` and `assets/github-social-preview.jpg` regenerated via KIE.ai gpt-image-2 (`scripts/regenerate-banner.py` is the reusable script). New banner reads "BENTERM" big white sans-serif left, retro CRT TV right with green phosphor halo, BEN red badge on the bezel, `benjamin@benterm ~` on the screen, glossy floor reflection. Same aesthetic as the old BenjaminTerm banner, fully rebranded.
- `assets/icon/terminal.png` (Linux 128px), `assets/windows/terminal.ico` (7 sizes 16–256), and `assets/macos/BENTERM.app/Contents/Resources/terminal.icns` (7 sizes 16–1024) all regenerated from `assets/icon/BENTERM.jpg` (the BEN red graphic).
- macOS bundle dir renamed `WezTerm.app` → `BENTERM.app`. Info.plist `CFBundleExecutable` case-fixed (`BENTERM-gui` → `benterm-gui`) so macOS can find the binary the package script writes.
- Linux integration files renamed and rewritten with `com.avalonreset.benterm` ids: `wezterm.desktop` → `benterm.desktop`, `wezterm.appdata.xml` → `benterm.appdata.xml`, `wezterm-nautilus.py` → `benterm-nautilus.py`, `open-wezterm-here` → `open-benterm-here`. `StartupWMClass` deliberately kept as `org.wezfurlong.wezterm` to match the unchanged Rust window class.
- Flatpak templates renamed to `com.avalonreset.benterm.*` with internals rewritten.
- 30+ `gen_*.yml` workflow trigger paths updated; `ci/generate-workflows.py` `TRIGGER_PATHS_UNIX` updated.
- Bundled distro config now ships as `benterm.lua` instead of `wezterm.lua` (installer + zip + macOS + Linux release scripts all updated).
- Legacy upstream SVGs (`wezterm-icon.svg`, `wezterm-ghifarit53-{1,2,3}.svg`) deleted.
- `assets/icon/benterm_screenshot.{jpg,webp}` deleted (had "WEZTERM-POWERED" text baked into the screenshot).
- README softened: "Built from WezTerm" badge dropped, "WezTerm (vanilla)" comparison column removed, alt text de-WezTerm'd. Upstream credit retained in the dedicated section per MIT requirements.

**macOS build.rs bug** (commit `4ccbe04a6`):
- `wezterm-gui/build.rs:165` was hardcoding `assets/macos/WezTerm.app/Contents/Info.plist` for a `UNUserNotificationCenter` plist copy. The bundle rename broke macOS builds with `panicked: ... copy ... assets/macos/WezTerm.app/Contents/Info.plist -> ... target/release/Info.plist: No such file or directory`. Linux + Windows weren't affected (cfg-gated). Caught after the first v2.0.0 push; fix landed and re-tagged.

**Process lessons** captured in [[Release Workflow Tax]]:
- The `benterm-release.yml` workflow's `prepare-release` step deletes the existing release on every run (`gh release delete --cleanup-tag --yes` at lines 51-53). On a single-platform code-bug failure, this forces a full all-platforms restart instead of allowing surgical re-run of the broken job. Workflow design choice, fixable in ~5 lines of YAML.
- No `sccache` configured for `benterm-release.yml` (the per-distro `gen_*.yml` workflows DO use it). Every release is cold compile, ~25 min wall clock per run.
- macOS Build step serializes 8 cargo invocations in a for-loop (4 binaries × 2 archs). Could be one `cargo build -p A -p B -p C -p D --target X --release` per arch, two parallel jobs.
- `gh release view v2.0.0 --json assets` is the ground-truth check for "is the installer ready," not the job status. Per-job upload steps complete before the job itself finishes cleanup; the job-level `in_progress` indicator can hide an already-uploaded artifact. **Cost**: I told the user "the installer doesn't exist yet" while it had already uploaded; they sat through unnecessary wait. Fixed for future status checks.

## 2026-04-27 - v1.4.5 Released: Cursor-Row Glow Stability + Claude Code OSC 9 Bridge

- Shipped [[Attention Trigger Lifecycle]] capturing the canonical OSC 9 / OSC 777 trigger design and the per-shell integration patterns. Documents anti-patterns to never re-introduce: idle-detection on `PaneOutput`, BEL → attention firing.
- Bundled `assets/shell-integration/benterm-claude-stop.ps1`, a Claude Code `Stop` hook helper that walks the parent process chain, calls `FreeConsole` + `AttachConsole` to attach to the conpty BENTERM reads (Claude detaches its hook subprocesses from the conpty by default, so a naive `printf > /dev/tty` or direct `CONOUT$` open lands on an ephemeral console nobody reads), and writes OSC 9 directly. Bridges Claude Code into the same per-pane attention path Codex uses natively.
- Switched `assets/shell-integration/benterm.ps1` from BEL emission to OSC 9 emission to match Mandatory Requirements M1.
- Cursor-row idle glow now live-tracks the cursor for ~1s after the ready signal (gives Claude Code / Codex time to do their final repaint and land the cursor on the actual input row), then locks. `mark_pane_input` clears the freeze. New `idle_text_glow_row` and `idle_text_glow_freeze_at` fields on `PaneState`.
- `warn_about_missing_glyphs` default flipped to `false` per the existing [[Notification Noise Policy]] - the policy was documented but the code default was still `true`. Common emoji codepoints in agent output (Claude Code, Codex) no longer fire the upstream missing-glyph toast.
- v1.4.4 also shipped today as a precursor: auto-discover `~/.benterm.lua` (and `<config-dir>/benterm.lua`, and `<exe-dir>/benterm.lua` on Windows) before the wezterm-named fallback. Plain BENTERM launches now load the user's full config without `--config-file` or a launcher script.

## 2026-04-23 - Banner Workflow Added And Sanitized

- Added [[README Banner Asset Workflow]] to document the public README banner path, `21:9` ratio, WebP delivery format, and local candidate promotion flow.
- Updated [[overview]], [[hot]], [[index]], and [[Concepts Index]] so the vault reflects the current public README surface.
- Removed remaining machine-specific path references from sound and validation notes.
- Confirmed local Obsidian workspace state remains ignored rather than tracked in the public vault.

## 2026-04-23 - Sound Cue Licensing Documented

- Traced BENTERM sound cues to the `sound-refresh-soft-cues` worktree.
- Added [[Sound Library Licensing]], [[Sound Grab Bag Attention System]], and [[Sound Cue Source Manifest]].
- Updated release hygiene notes so bundled sound cues keep their README, CC0 license note, and per-file source manifest.
- Verified `BENTERM-windows-v1.4.0.zip`, `BENTERM-windows-v1.4.1.zip`, and a local installed BENTERM build use `benterm-soft-cues` and do not include the old `kenney-interface` prototype directory.

## 2026-04-23 - README Vision Updated

- Updated README positioning around vibe coding, idle glow cues, pane-aware notifications, soft randomized attention sounds, fresh theme rotation, and attention switching.
- Mirrored the product vision in [[overview]] and [[BENTERM]].
- Later sound tracing found the implementation and generated asset pack in the `sound-refresh-soft-cues` worktree.

## 2026-04-23 - Hyper Yap Boundary Recorded

- Added [[BENTERM and Hyper Yap Boundary]].
- Captured decision that BENTERM owns smart terminal defaults while Hyper Yap owns dictated-block undo and wrong-window recovery.
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

- Created repo-local Obsidian vault at `BENTERM-Wiki`.
- Seeded repository-mode pages for release readiness, packaging, terminal defaults, release flow, decisions, and open questions.
- Referenced repository docs in `.raw/.manifest.json`.
