---
type: concept
title: "Sound Grab Bag Attention System"
created: 2026-04-23
updated: 2026-04-23
status: active
tags:
  - sounds
  - notifications
  - attention
related:
  - "[[Notification Noise Policy]]"
  - "[[Sound Library Licensing]]"
sources:
  - "[[Sound Cue Source Manifest]]"
---

# Sound Grab Bag Attention System

BenjaminTerm uses soft randomized sound cues as part of its sensory workflow layer for vibe coding. The intent is not to make notifications loud. The intent is to let different terminal windows acquire subtle audible identity during long multi-window coding sessions.

## Product Role

- Visual glow shows when a terminal has gone quiet and is ready.
- Pane-aware notifications identify the exact terminal surface that needs attention.
- Soft randomized sound cues give attention events an audible identity.
- Theme rotation gives sessions visual identity.

Together these cues make it easier to reacquire the right terminal without reading every window.

## Implementation Notes From Sound Refresh Work

- Sound refresh changes were traced from the `sound-refresh-soft-cues` branch during release validation.
- Runtime file: `wezterm-gui/src/attention_sound.rs`
- Sound pack path: `assets/sounds/benjaminterm-soft-cues/`
- Package scripts copy the sound pack into Windows and macOS release artifacts.

## Release Rule

If sound cues ship in a release, their README, CC0 license note, and per-file source manifest must ship with them.

Only attribute the sounds that ship. The rejected `kenney-interface` prototype set should stay out of release artifacts and out of attribution notices unless it reappears in a shipped package.
