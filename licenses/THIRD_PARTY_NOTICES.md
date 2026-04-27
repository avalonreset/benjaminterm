BENTERM Third-Party Notices
================================

BENTERM is a custom distribution of WezTerm. This repository and its
release artifacts include third-party components with their own license terms.

Primary project license
-----------------------
- WezTerm/BENTERM codebase license: MIT
- See: `LICENSE.md`

Bundled font
------------
- 0xProto
  - License: SIL Open Font License 1.1
  - License text: `assets/fonts/LICENSE_0XPROTO.txt`

Windows runtime-distributed components
--------------------------------------
- ANGLE libraries (`libEGL.dll`, `libGLESv2.dll`)
  - License text: `licenses/ANGLE.md`

Bundled sound cues
------------------
- BENTERM soft cue WAV files under
  `assets/sounds/benterm-soft-cues/` are generated from CC0 sound sources:
  - Kenney UI Audio
    - Source: https://kenney.nl/assets/ui-audio
    - License: Creative Commons CC0
  - ObsydianX Interface SFX Pack 1
    - Source: https://obsydianx.itch.io/interface-sfx-pack-1
    - License: Creative Commons Zero v1.0 Universal
- Per-file provenance is preserved in:
  - `assets/sounds/benterm-soft-cues/SOURCE_MANIFEST.json`
- Source and license notes are preserved in:
  - `assets/sounds/benterm-soft-cues/README.md`
  - `assets/sounds/benterm-soft-cues/LICENSE_CC0.txt`
- Do not attribute the old `kenney-interface` prototype set unless it appears
  in the artifact being shipped. Current `v1.4.0` and `v1.4.1` packages use
  `benterm-soft-cues` and do not include `kenney-interface`.

Practical compliance note
-------------------------
When redistributing BENTERM binaries/installers, include:
- `LICENSE.md`
- this file (`licenses/THIRD_PARTY_NOTICES.md`)
- `licenses/ANGLE.md`
- `assets/fonts/LICENSE_0XPROTO.txt`
- `assets/sounds/benterm-soft-cues/README.md`
- `assets/sounds/benterm-soft-cues/LICENSE_CC0.txt`
- `assets/sounds/benterm-soft-cues/SOURCE_MANIFEST.json`
