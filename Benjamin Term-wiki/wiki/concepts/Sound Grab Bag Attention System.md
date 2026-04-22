---
type: concept
title: "Sound Grab Bag Attention System"
created: 2026-04-22
updated: 2026-04-22
tags:
  - sound
  - notifications
  - productivity
status: prototyping
related:
  - "[[Agent Completion Attention Flow]]"
  - "[[Sound Library Licensing]]"
  - "[[Tab Attention Indicator]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "cargo check -p wezterm-gui; cargo build -p wezterm-gui"
---

# Sound Grab Bag Attention System

The Sound Grab Bag Attention System assigns each agent surface a subtle, tasteful sound identity. The surface is the originating pane/tab, not only the outer OS window. When an agent finishes and is ready for another prompt, BenjaminTerm plays that surface's assigned sound and triggers the visual attention path.

Product thesis:

- Users often run several agent terminals at once.
- Users may also run several agent tabs inside one BenjaminTerm window.
- Visual scanning is slower than recognizing a recurring sound.
- Per-pane/tab sound identity can become a Pavlovian cue for which task needs attention.

Guardrails:

- Sounds must be subtle and short.
- The feature should be configurable and easy to disable.
- The sound library must be legally redistributable.
- Sounds should map to actionable ready states, not every minor terminal event.
- Avoid novelty sounds that become annoying during long coding sessions.

Prototype status:

- Source library: Kenney Interface Sounds.
- License: CC0.
- Asset count: 100 sounds.
- Prototype format: mono 44.1 kHz 16-bit WAV.
- Runtime behavior: each mux pane receives one sound from a runtime shuffle bag on first attention event.
- Assignment lifetime: the pane keeps the same sound until it closes.
- Bag behavior: sounds are not repeated for new pane assignments until the full pack has been used.
- Focus rule: the sound plays for the agent-ready notification even when the originating pane/window is already focused and no Windows toast is shown.
- Focus-reporting rule: BenjaminTerm reports focus-tracking panes as unfocused to terminal applications so agent CLIs do not suppress ready notifications just because the user is already focused on the pane.
- Toast audio rule: Windows toast XML is silent; BenjaminTerm's own sound cue is the only intended notification audio.
- Next refinement: expose sound enable/disable and volume/style controls.
