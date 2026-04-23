---
type: meta
title: "Wiki Operating Guide"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - wiki
  - operations
related:
  - "[[overview]]"
---

# Wiki Operating Guide

## Best Practices

- Start every wiki task by reading [[hot]].
- Use [[index]] to avoid duplicate pages.
- Follow [[Wiki Maintenance Flow]] for durable changes.
- Put source facts in source notes and durable conclusions in wiki notes.
- Keep notes small enough that one future answer can cite them cleanly.
- Add contradictions as callouts instead of silently overwriting old claims.
- Update [[log]] newest-first after meaningful changes.
- Keep [[hot]] under 500 words.

## Recommended Obsidian Setup

- Open `<repo-root>\BenjaminTerm-Wiki` as the vault root.
- Use `_templates/` as the Templates plugin folder.
- Store pasted images and files in `_attachments/`.
- Use `wiki/canvases/main.canvas` as the visual map.

## Suggested Plugins

These are optional. The vault works without them.

- Templates: use the files in `_templates/`.
- Canvas: view `wiki/canvases/main.canvas`.
- Dataview or Bases: useful later for status dashboards.

## Codex Maintenance Prompt

Use this when you want Codex to update the vault:

```text
Use the codex-obsidian skill. Read BenjaminTerm-Wiki/wiki/hot.md and BenjaminTerm-Wiki/wiki/index.md first, then update the smallest relevant note. Keep hot, index, and log current.
```
