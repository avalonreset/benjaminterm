---
type: decision
title: "Soft Cue Pack Refresh"
created: 2026-04-22
updated: 2026-04-22
tags:
  - sound
  - release
  - packaging
status: active
related:
  - "[[Sound Grab Bag Attention System]]"
  - "[[Sound Library Licensing]]"
  - "[[Branding And Packaging]]"
sources:
  - "https://kenney.nl/assets/ui-audio"
  - "https://obsydianx.itch.io/interface-sfx-pack-1"
validation: "cargo check -p wezterm-gui; cargo build --release -p wezterm -p wezterm-gui -p wezterm-mux-server -p strip-ansi-escapes; package script; ffmpeg volumedetect"
---

# Soft Cue Pack Refresh

Decision: BENTERM's bundled agent-attention sounds should use a re-sourced, calmer CC0 cue set. The previous `kenney-interface` cue library is removed from the app, packaging scripts, and runtime fallback paths.

## Current Pack

- Directory: `assets/sounds/benterm-soft-cues/`
- Packaged review directory: `dist/BENTERM-windows-v2026.04.22-soft-cues-curated/sounds/benterm-soft-cues/`
- Portable zip: `dist/BENTERM-windows-v2026.04.22-soft-cues-curated.zip`
- Zip SHA256: `c9cd5e0154cd14a5c2ff6d57b81810fb094268abdeccdbe1b9d0b0b7c24e9140`
- Manifest: `assets/sounds/benterm-soft-cues/SOURCE_MANIFEST.json`

## Source Split

- 51 files from Kenney UI Audio.
- 33 files from ObsydianX Interface SFX Pack 1.
- 84 WAV files total after curation.
- License: CC0 for both sources.

## Curation Rules

- Do not include any files from the old Kenney Interface Sounds / `kenney-interface` pack.
- Remove cues that become effectively inaudible after remastering instead of boosting them back into the pool.
- Keep the sound set focused on short UI/attention cues, not novelty, horror, impact, voice, or broad game SFX.
- User ear review is authoritative because automated analysis cannot judge taste or startle factor.

## Remastering

- WAV, mono, 44.1 kHz, 16-bit PCM.
- Per-file peak normalized to approximately `-14 dB`.
- Latest packaged measurement:
  - Count: 84 WAVs.
  - Average max peak: `-14.0 dB`.
  - Quietest max peak: `-14.0 dB`.
  - Loudest max peak: `-13.9 dB`.

Peak normalization prevents one cue from being dramatically louder than another. Perceived loudness can still vary with timbre, duration, and transient shape, so listening review remains required.

## Implementation Notes

- Runtime discovery now checks `benterm-soft-cues` and no longer falls back to `kenney-interface`.
- Windows installer, portable packaging, deploy script, and GitHub release workflow copy `assets/sounds/benterm-soft-cues`.
- `LICENSE.md` points to `assets/sounds/benterm-soft-cues/LICENSE_CC0.txt`.

## Validation

- `cargo check -p wezterm-gui` passed with existing warnings.
- `cargo build --release -p wezterm -p wezterm-gui -p wezterm-mux-server -p strip-ansi-escapes` passed with existing warnings.
- `ci/package-benterm-windows.ps1 -TagName v2026.04.22-soft-cues-curated` produced the portable zip.
- `git diff --check` passed aside from expected Windows line-ending warnings.
- Packaged manifest confirms 51 Kenney files and 33 ObsydianX files.

## Remaining Work

- User should listen through the packaged review directory and identify any still-annoying or still-too-quiet cues.
- Inno Setup is not installed locally, so the installer was not rebuilt on this machine.
- If installer rebuild is needed locally, install Inno Setup 6 or build through GitHub Actions.
