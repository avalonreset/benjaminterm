---
type: question
title: "Open Release Questions"
created: 2026-04-22
updated: 2026-04-23
status: open
tags:
  - questions
  - release
related:
  - "[[Release Readiness 2026-04-22]]"
  - "[[Sound Library Licensing]]"
---

# Open Release Questions

## macOS Signing

Should the first macOS release be signed and notarized, or is an unsigned zip acceptable for the first cross-platform release candidate?

## Announcement Timing

Should BENTERM be announced only after all three platform workflows upload artifacts successfully?

## Package Managers

When should Homebrew, Linuxbrew, Flathub, WinGet, and Gemfury be enabled? Current recommendation: after GitHub releases are proven.

## Sound Cue Merge

Before a release that advertises soft randomized attention sounds, confirm the `sound-refresh-soft-cues` worktree changes are merged into the release branch and the CC0 source manifest ships with artifacts.

## Resolved During This Session

- Dictated-block undo should be handled in Hyper Yap, not expanded inside BENTERM.
- The wiki should live in the main repository after sanitization, but not inside installers yet.
- Sound cue licensing was traced to CC0 source packs and documented in [[Sound Library Licensing]].
