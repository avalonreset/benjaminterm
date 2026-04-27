---
type: meta
title: "Wiki Lint Report 2026-04-22"
created: 2026-04-22
updated: 2026-04-23
status: clean
tags:
  - lint
  - wiki
---

# Wiki Lint Report 2026-04-22

## Result

Clean after scaffold expansion and Hyper Yap boundary update.

## Checks

- Frontmatter present on created markdown notes.
- Seed wikilinks checked for missing target pages and canvas files.
- Core maintenance pages present: [[hot]], [[index]], [[log]], [[overview]].
- Folder indexes added for active wiki areas.
- Canvas JSON and Obsidian bookmark JSON parse successfully.
- Public-release scan found local absolute paths only; seed notes now use `<repo-root>`.
- 2026-04-23 recheck: wikilinks OK, JSON OK, frontmatter OK, hot cache under 500 words.
- 2026-04-23 sanitization scan only hits [[Public Release Sanitization]], where the scan command and generic token names are documented.

## Known Gaps

- Source manifest references repo docs but does not yet store content hashes.
- Dependency map is a seed overview, not a full lockfile audit.
- Canvas is a starter map and should evolve as the project grows.
