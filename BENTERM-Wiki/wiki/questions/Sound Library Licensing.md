---
type: question
title: "Sound Library Licensing"
created: 2026-04-23
updated: 2026-04-23
status: answered
tags:
  - questions
  - licensing
  - sounds
related:
  - "[[Sound Grab Bag Attention System]]"
  - "[[Public Release Sanitization]]"
sources:
  - "[[Sound Cue Source Manifest]]"
---

# Sound Library Licensing

## Answer

The BENTERM soft cue pack is legally low-friction for redistribution because the tracked source materials are CC0.

- Kenney UI Audio: Creative Commons CC0.
- ObsydianX Interface SFX Pack 1: Creative Commons Zero v1.0 Universal.
- The generated BENTERM pack contains 84 WAV files: 51 from Kenney UI Audio and 33 from ObsydianX Interface SFX Pack 1.
- The old Kenney Interface Sounds pack is intentionally excluded from the generated cue pack.
- Do not attribute the old `kenney-interface` prototype pack unless it actually ships in an artifact.

CC0 does not require attribution, but BENTERM should still ship source notes as a practical compliance and provenance measure.

## Required Release Hygiene

When sound cues are bundled, release artifacts should preserve:

- `assets/sounds/benterm-soft-cues/README.md`
- `assets/sounds/benterm-soft-cues/LICENSE_CC0.txt`
- `assets/sounds/benterm-soft-cues/SOURCE_MANIFEST.json`

Also keep `licenses/THIRD_PARTY_NOTICES.md` current with the two source pages and the CC0 status.

## Validation

Verified from the `sound-refresh-soft-cues` branch and official source pages:

- https://kenney.nl/assets/ui-audio
- https://obsydianx.itch.io/interface-sfx-pack-1

Package verification from the release inspection work:

- `BENTERM-windows-v1.4.0.zip`: 84 WAV files under `sounds/benterm-soft-cues`, 0 old `kenney-interface` entries.
- `BENTERM-windows-v1.4.1.zip`: 84 WAV files under `sounds/benterm-soft-cues`, 0 old `kenney-interface` entries.
- A local installed BENTERM build was also checked and matched the same result.
