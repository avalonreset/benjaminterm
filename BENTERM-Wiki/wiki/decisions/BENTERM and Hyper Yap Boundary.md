---
type: decision
title: "BENTERM and Hyper Yap Boundary"
created: 2026-04-23
updated: 2026-04-23
status: accepted
tags:
  - decisions
  - hotkeys
  - hyper-yap
related:
  - "[[Terminal Defaults]]"
  - "[[Smart Copy Interrupt]]"
  - "[[Release Readiness 2026-04-22]]"
---

# BENTERM and Hyper Yap Boundary

BENTERM is allowed to be opinionated, but it should own terminal-level behavior rather than personal automation semantics.

## BENTERM Owns

- Smart terminal copy behavior: selected text copies; no selection sends interrupt.
- Normal terminal paste bindings by platform.
- Quiet notification defaults.
- Release packaging, branding, and terminal defaults that are useful without Hyper Yap.

## Hyper Yap Owns

- Dictation insertion strategy.
- Guaranteed undo for dictated blocks.
- Active-window targeting and wrong-window recovery.
- Any behavior that depends on knowing a chunk came from speech-to-text.

## Rationale

Hyper Yap knows user intent. BENTERM usually only sees terminal input bytes and pane state. Undoing a dictated block requires knowing that a specific chunk was inserted as one operation; that belongs in Hyper Yap. BENTERM can still provide conservative terminal ergonomics that work whether Hyper Yap is installed or not.

## Release Impact

The smart `Ctrl+C` fix remains in BENTERM and should be included in the next release if releasing updated artifacts. The broader dictated-block undo work should be implemented in Hyper Yap, not expanded further inside BENTERM.
