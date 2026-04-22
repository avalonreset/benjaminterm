---
type: flow
title: "Preview Testing Flow"
created: 2026-04-22
updated: 2026-04-22
tags:
  - testing
  - preview
status: active
related:
  - "[[Rebuild Release State]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Preview Testing Flow

Preview goals:

- Keep current BenjaminTerm and upstream WezTerm untouched.
- Test config-only behavior through the installed WezTerm binary when possible.
- Test source-level behavior through a source-built `wezterm-gui.exe`.

Toast pulse testing requires a source-built preview because copied old binaries do not contain the new pulse code.

