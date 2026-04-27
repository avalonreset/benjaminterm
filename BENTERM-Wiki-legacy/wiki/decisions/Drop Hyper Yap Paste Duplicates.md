---
type: decision
title: "Drop Hyper Yap Paste Duplicates"
created: 2026-04-22
updated: 2026-04-22
tags:
  - hyperyap
  - paste
  - decision
status: accepted
related:
  - "[[Hyper Yap]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: "manual workflow check"
---

# Drop Hyper Yap Paste Duplicates

## Decision

Do not carry forward BENTERM smart paste, image paste, OpenWhispr/Claude paste helpers, or paste undo/redo.

## Rationale

[[Hyper Yap]] already owns those workflows. Duplicating them inside BENTERM increases risk and makes the terminal harder to reason about.

