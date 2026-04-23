---
type: concept
title: "Notification Noise Policy"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - notifications
  - defaults
related:
  - "[[Terminal Defaults]]"
sources:
  - "[[Project Documentation]]"
---

# Notification Noise Policy

BenjaminTerm should avoid noisy desktop notifications by default.

## Current Policy

- Missing-glyph warnings are suppressed by default for BenjaminTerm.
- Users can still opt into upstream-style warning behavior through configuration.
- Focusable notifications remain useful when they connect the user back to the originating terminal context.

## Why

Default desktop notifications should be actionable. A repeated missing-glyph warning during normal terminal use is not useful enough to justify interrupting the user.
