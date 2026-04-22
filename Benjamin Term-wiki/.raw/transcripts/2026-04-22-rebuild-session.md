---
source_id: rebuild-session-2026-04-22
captured: 2026-04-22
kind: conversation-summary
status: immutable
---

# BenjaminTerm Rebuild Session Summary

The rebuild starts from upstream WezTerm and keeps BenjaminTerm changes small and reviewable.

Kept features include 0xProto as bundled default font, the pure-black theme shuffle-bag system, larger default scale, fancy tab bar hidden when only one tab exists, BEN branding for release, and Windows toast click-to-focus.

Dropped or deferred features include smart paste, image paste, OpenWhispr/Claude paste helpers, paste undo/redo, ConPTY resize hacks, renderer default hacks, and broad terminal internals changes.

The Windows notification system uses upstream OSC 9 / OSC 777 terminal notifications. BenjaminTerm adds click-to-focus routing so a notification click focuses the pane/window that emitted it. It also adds a brief cyan border pulse to make the focused terminal visually obvious.

The sound grab-bag idea proposes assigning each terminal window a subtle unique sound effect so agent completion can be recognized without visual scanning. This should remain tasteful, configurable, and tied to actionable agent-ready states.

