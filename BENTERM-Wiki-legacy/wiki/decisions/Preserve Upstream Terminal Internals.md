---
type: decision
title: "Preserve Upstream Terminal Internals"
created: 2026-04-22
updated: 2026-04-22
tags:
  - upstream
  - decision
status: accepted
related:
  - "[[WezTerm]]"
sources:
  - "[[Rebuild Session Summary]]"
validation: pending
---

# Preserve Upstream Terminal Internals

## Decision

Avoid carrying forward old ConPTY resize, reflow, renderer-default, and broad terminal internals changes.

## Rationale

Those areas are high-risk and likely contributed to the old fork feeling scrambled. BENTERM should be special through identity and workflow affordances, not fragile terminal-core changes.

