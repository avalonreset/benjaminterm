---
type: meta
title: "Benjamin Term Wiki Instructions"
created: 2026-04-22
updated: 2026-04-22
tags:
  - benjaminterm
  - codex
status: active
---

# Benjamin Term Wiki Instructions

This vault is the persistent project wiki for [[BenjaminTerm]], a careful rebuild of BenjaminTerm from upstream [[WezTerm]].

## Operating Rules

- Read `wiki/hot.md` first, then `wiki/index.md`.
- Keep `.raw/` as immutable source material.
- Keep synthesized notes under `wiki/`.
- Update existing notes before creating duplicates.
- Use Obsidian wikilinks for internal references.
- Keep `wiki/hot.md`, `wiki/index.md`, `wiki/log.md`, and `wiki/overview.md` current after meaningful work.
- Record release-impacting validation status in decision notes.
- Do not store private user data unless explicitly requested.

## Project Constraints

- Preserve upstream WezTerm internals unless a BenjaminTerm feature has a clear workflow reason.
- Treat terminal core, ConPTY, mux, screen, resize, and text rendering changes as high risk.
- Keep [[Hyper Yap]] responsible for speech, clipboard, paste, and image-paste workflows.
- Keep BenjaminTerm focused on terminal identity, theme/font defaults, agent attention, and release packaging.

