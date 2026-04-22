---
type: flow
title: "Agent Completion Attention Flow"
created: 2026-04-22
updated: 2026-04-22
tags:
  - agents
  - notifications
  - productivity
status: developing
related:
  - "[[Codex]]"
  - "[[Claude]]"
  - "[[Sound Grab Bag Attention System]]"
  - "[[Tab Attention Indicator]]"
  - "[[Idle Text Glow Cue]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Agent Completion Attention Flow

Goal: when an agent is ready for another prompt, BenjaminTerm should make the correct terminal easy to identify and enter.

Current flow:

1. Agent emits a terminal notification.
2. BenjaminTerm immediately plays the originating pane/tab's attention sound.
3. BenjaminTerm immediately triggers [[Attention Pulse]] for the containing window.
4. BenjaminTerm marks the originating tab through [[Tab Attention Indicator]].
5. The ready event arms [[Idle Text Glow Cue]], which invites the next prompt in the content area.
6. If the originating pane is not already focused, BenjaminTerm shows a Windows toast.
7. User clicks the toast.
8. BenjaminTerm focuses the originating pane/window and triggers visual attention again without replaying the sound.
9. When the user sends the next response into that pane, BenjaminTerm dismisses the pane's outstanding toast and clears the idle glow until the next ready event.

Important rule: the Pavlovian sound cue is not dependent on whether a Windows toast is shown. BenjaminTerm also reports focus-tracking panes as unfocused to terminal applications so agent CLIs keep emitting ready notifications even while the user is actively looking at the pane.

Sound/visual split:

- Agent-ready notification: sound plus visual pulse.
- Toast click: visual pulse only.
- Focused pane: sound plus visual pulse, no Windows toast.
- Background tab or other window: sound plus visual pulse plus a silent, clickable Windows reminder toast.
- User response input: clears the pane-scoped Windows toast reminder and disarms the idle text glow until the next ready event.

Nagging toast rule:

- Each agent-ready event gets a fresh notification tag so Windows treats it as a new visible reminder.
- All outstanding reminders for the pane share a pane-scoped group so one response clears that pane's queue.
- Focusable agent-ready toasts use `scenario="reminder"` and a `Focus` action button to get as close as Windows allows to an on-screen nag.

Implemented sound identity behavior:

1. Each pane/tab session receives a shuffle-bag sound identity from [[Sound Grab Bag Attention System]].
2. The identity persists for that pane while it is alive.
3. When the pane closes, its assignment is removed.

Next refinement:

1. User can disable sounds or choose quieter/louder sound packs.
2. The release package includes the sound license notice.
