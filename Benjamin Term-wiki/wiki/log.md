---
type: meta
title: "Operation Log"
created: 2026-04-22
updated: 2026-04-22
tags:
  - benjaminterm
  - log
status: active
---

# Operation Log

## 2026-04-21 - Reminder Toast Mode

- Added `scenario="reminder"` support to the Windows toast backend.
- Focusable agent-ready toasts now include a `Focus` action button in addition to body click-to-focus.
- Kept silent toast audio, fresh per-event tags, and pane-scoped group cleanup.
- Updated [[Microsoft Toast Notification System]], [[Toast Notification Backend]], [[Windows Toast Click-To-Focus]], [[Agent Completion Attention Flow]], and [[hot]].

## 2026-04-21 - Microsoft Toast Research And Nagging Tags

- Added [[Microsoft Toast Notification System]] research note from Microsoft documentation.
- Captured `duration`, `scenario`, `Tag`, `Group`, headers, Notification Center behavior, and UX guidance.
- Changed the BenjaminTerm design from stable per-pane toast tags to fresh per-event tags plus pane-scoped groups.
- Updated [[Toast Notification Backend]], [[Windows Toast Click-To-Focus]], [[Agent Completion Attention Flow]], [[hot]], and [[index]].
- Validated with `cargo check -p wezterm-gui`, `git diff --check`, and wiki link lint.

## 2026-04-22 - Input Clears Agent Toasts

- Added pane-scoped toast tags for focusable Windows notifications.
- Added a dismiss path that removes the pane's outstanding toast when the user sends keyboard input, clipboard paste, dropped text/files, `SendString`, or `SendKey`.
- Updated [[Windows Toast Click-To-Focus]], [[Toast Notification Backend]], [[Agent Completion Attention Flow]], [[Rebuild Release State]], and [[hot]].
- Validated with `cargo check -p wezterm-gui`.

## 2026-04-22 - Attention Checkpoint Ready

- Confirmed the agent-ready attention system is working well in preview: per-pane sound cue, theme-aware visual pulse, silent Windows toast, and click-to-focus.
- Set BenjaminTerm's config layer to suppress toasts from the already-focused pane while still allowing background-tab and other-window toasts.
- Kept persistent Windows toasts as the current queue-like reminder model for terminals that need attention.
- Updated [[hot]], [[Attention Pulse]], [[Windows Toast Click-To-Focus]], and [[Agent Completion Attention Flow]].

## 2026-04-22 - Theme-Aware Attention Color

- Replaced hard-coded cyan attention color with a saturated accent selected from the active palette.
- The selector prefers bright ANSI accent colors and avoids gray/white foreground-like candidates.
- Updated [[Attention Pulse]] and [[Tab Attention Indicator]].

## 2026-04-22 - Toast Click Visual-Only Cue

- Split attention behavior into explicit sound-plus-visual and visual-only paths.
- Agent-ready notifications play the assigned sound and trigger the visual cue.
- Toast clicks focus the pane/window and retrigger the visual cue without replaying the sound.

## 2026-04-22 - Focused-Pane Sound Cue Requirement

- Updated the agent-ready path so the Pavlovian sound cue is not tied to whether a Windows toast is shown.
- Captured the rule that sound still fires when the originating pane/window is already focused.
- Restored the known-working frontend sound trigger after the terminal-window mux handler proved unreliable for the cue.
- Added focus-reporting behavior so terminal applications that request focus tracking are told the pane is unfocused, encouraging agent CLIs to emit ready notifications even while the pane is active.
- Made Windows toast XML silent via `<audio silent="true"/>` so the built-in Windows notification sound does not compete with BenjaminTerm's cue.

## 2026-04-22 - Sound Shuffle Bag Implemented

- Replaced deterministic pane-id hashing with a runtime sound shuffle bag.
- Each pane receives one sound on first attention event and keeps it until pane removal.
- Closing a pane removes the assignment without immediately recycling that sound ahead of the bag.
- Updated [[Sound Grab Bag Attention System]], [[Agent Completion Attention Flow]], and [[hot]].

## 2026-04-22 - Pane And Tab Attention Model Captured

- Updated [[Sound Grab Bag Attention System]] so sound identity belongs to the originating pane/tab, not only the OS window.
- Added [[Tab Attention Indicator]] for background-tab ready states.
- Updated [[Agent Completion Attention Flow]], [[Attention Pulse]], [[overview]], and [[hot]] to reflect the layered attention model.
- Verified with `cargo check -p wezterm-gui`, `cargo build -p wezterm-gui`, `git diff --check`, and wiki link lint.

## 2026-04-22 - Sound Grab-Bag Prototype Started

- Selected Kenney Interface Sounds as the prototype sound source.
- Confirmed OpenGameArt lists the pack as CC0.
- Converted 100 OGG sounds to WAV for Windows native playback.
- Updated [[Sound Grab Bag Attention System]], [[Agent Completion Attention Flow]], and [[Sound Library Licensing]].

## 2026-04-22 - Vault Scaffolded

- Created `Benjamin Term-wiki` using the Codex Obsidian LLM Wiki pattern.
- Seeded repository-mode wiki structure.
- Filed initial notes for rebuild state, key decisions, toast focus, attention pulse, and sound grab-bag concept.
- Updated hot cache and index.
