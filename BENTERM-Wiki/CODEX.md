---
type: instructions
title: "BENTERM Wiki Instructions"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - codex
  - wiki
---

# BENTERM Wiki Instructions

This vault is the project wiki for [[BENTERM]].

Use repository mode:

- Keep immutable source material in `.raw/`.
- Keep synthesized notes in `wiki/`.
- Update existing notes before creating duplicates.
- Use Obsidian wikilinks such as `[[Release Readiness 2026-04-22]]`.
- Keep `wiki/hot.md`, `wiki/index.md`, `wiki/log.md`, and `wiki/overview.md` current after meaningful work.
- Record validation status for release-impacting changes.
- Do not store private user data unless explicitly asked.

For source-code work, the repository root is one directory up from this vault:

```text
<repo-root>
```

The repo currently contains substantial release-candidate changes. Do not revert unrelated dirty files.

## Default Maintenance Loop

For any meaningful project update:

1. Read `wiki/hot.md`.
2. Read `wiki/index.md`.
3. Update the smallest relevant note.
4. Add or update source references when facts come from repo files.
5. Update `wiki/hot.md`.
6. Add a newest-first entry to `wiki/log.md`.
7. Run a wikilink sanity check before finishing.

## Best Practices

- Prefer one durable idea per note.
- Use note templates from `_templates/`.
- Keep release facts dated and validated.
- Put uncertain items in `wiki/questions/`, not in definitive pages.
- Use `wiki/canvases/main.canvas` as a visual map only; keep real knowledge in markdown notes.

## Public Release Sanitization

This vault is intended to be publishable with the project. Before committing or releasing it:

- Remove local absolute paths and use `<repo-root>` or relative paths.
- Do not include usernames, machine names, home directories, private repo URLs, API keys, secrets, tokens, credentials, `.env` values, or private screenshots.
- Keep `.raw/private/` and `_attachments/private/` out of git.
- Run a vault scan for common sensitive patterns.
- Update [[Public Release Sanitization]] when sanitization status changes.
