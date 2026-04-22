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
5. If the originating pane is not already focused, BenjaminTerm shows a Windows toast.
6. User clicks the toast.
7. BenjaminTerm focuses the originating pane/window and triggers visual attention again without replaying the sound.
8. When the user sends the next response into that pane, BenjaminTerm dismisses the pane's outstanding toast.

Important rule: the Pavlovian sound cue is not dependent on whether a Windows toast is shown. BenjaminTerm also reports focus-tracking panes as unfocused to terminal applications so agent CLIs keep emitting ready notifications even while the user is actively looking at the pane.

Sound/visual split:

- Agent-ready notification: sound plus visual pulse.
- Toast click: visual pulse only.
- Focused pane: sound plus visual pulse, no Windows toast.
- Background tab or other window: sound plus visual pulse plus a silent, clickable Windows toast.
- User response input: clears the pane-scoped Windows toast reminder.

Implemented sound identity behavior:

1. Each pane/tab session receives a shuffle-bag sound identity from [[Sound Grab Bag Attention System]].
2. The identity persists for that pane while it is alive.
3. When the pane closes, its assignment is removed.

Next refinement:

1. User can disable sounds or choose quieter/louder sound packs.
2. The release package includes the sound license notice.
