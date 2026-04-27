---
type: question
title: "Sound Library Licensing"
created: 2026-04-22
updated: 2026-04-23
tags:
  - sound
  - licensing
status: answered
related:
  - "[[Sound Grab Bag Attention System]]"
  - "[[Soft Cue Pack Refresh]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Sound Library Licensing

Open question: what sound sources can BENTERM legally bundle?

Requirements:

- Redistributable in a commercial-friendly open-source project.
- Short, subtle, tasteful UI sounds.
- Clear license text included in release artifacts.
- No unclear marketplace or sample-pack licensing.

Release answer:

- Kenney UI Audio is suitable for part of the quieter default cue pack.
- ObsydianX Interface SFX Pack 1 is suitable for the remaining UI/interface cues.
- Both sources are CC0.
- The current generated pack contains 84 WAV files: 51 from Kenney UI Audio and 33 from ObsydianX Interface SFX Pack 1.
- Extremely low-energy ObsydianX cues were removed from the packaged set after remastering.
- The old Kenney Interface Sounds pack is excluded from the replacement pack.
- Do not attribute the old `kenney-interface` prototype pack unless it actually ships in an artifact.
- BENTERM should ship a source/credits note with release artifacts.
- Current source mapping is captured in `assets/sounds/benterm-soft-cues/SOURCE_MANIFEST.json`.
- Current third-party notice file is `licenses/THIRD_PARTY_NOTICES.md`.

Sources:

- https://kenney.nl/assets/ui-audio
- https://obsydianx.itch.io/interface-sfx-pack-1

Package verification:

- `BENTERM-windows-v1.4.0.zip`: 84 WAV files under `sounds/benterm-soft-cues`, 0 old `kenney-interface` entries.
- `BENTERM-windows-v1.4.1.zip`: 84 WAV files under `sounds/benterm-soft-cues`, 0 old `kenney-interface` entries.
- Installed `C:/Program Files/BENTERM/sounds/benterm-soft-cues`: 84 WAV files.
- Installed `C:/Program Files/BENTERM/sounds/kenney-interface`: absent.
