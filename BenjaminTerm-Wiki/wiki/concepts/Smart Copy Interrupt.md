---
type: concept
title: "Smart Copy Interrupt"
created: 2026-04-22
updated: 2026-04-22
status: implemented
tags:
  - copy-paste
  - keybindings
  - release
related:
  - "[[Terminal Defaults]]"
  - "[[Release Readiness 2026-04-22]]"
sources:
  - "[[Repository Source Map]]"
---

# Smart Copy Interrupt

BenjaminTerm uses smart `Ctrl+C` behavior:

- If the active pane has selected text, `Ctrl+C` copies the selection and clears it.
- If there is no selection, `Ctrl+C` is sent through to the pane as normal interrupt input.

This keeps normal Windows-style copy behavior without removing terminal interrupt behavior.

## Implementation

- Native action: `CopySelectionOrSendKey`.
- Default binding: `Ctrl+C`.
- Vibe config also keeps a Lua callback fallback and mapped-key binding.
