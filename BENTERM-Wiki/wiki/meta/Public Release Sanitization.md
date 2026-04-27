---
type: meta
title: "Public Release Sanitization"
created: 2026-04-22
updated: 2026-04-23
status: active
tags:
  - sanitization
  - public-release
  - security
related:
  - "[[Wiki Operating Guide]]"
  - "[[Wiki Lint Report 2026-04-22]]"
---

# Public Release Sanitization

This wiki is intended to be useful to future developers and safe to publish with the project.

## Do Not Publish

- API keys, access tokens, passwords, credentials, private keys, signing keys, recovery codes, or `.env` values.
- Local usernames, machine names, home directories, private drive layouts, or private absolute paths.
- Private screenshots, private transcripts, private logs, or raw conversation dumps.
- Private repo URLs or provider account details that are not already public project facts.

## Safe Patterns

- Use `<repo-root>` instead of a local absolute path.
- Use relative repo paths such as `ci/deploy.sh`.
- Describe external credentials generically, such as "Apple Developer credentials" or "`GH_PAT` secret," without including values.
- Keep durable, project-level decisions in `wiki/`; keep private source material out of the vault or under ignored private folders.

## Pre-Publish Scan

Run from the repository root:

```powershell
rg -n "C:\\|E:\\|Users\\|API[_ -]?KEY|SECRET|TOKEN|PASSWORD|BEGIN (RSA|OPENSSH|PRIVATE)|sk-[A-Za-z0-9]|github_pat_|ghp_|gho_|AIza|AKIA|-----BEGIN" BENTERM-Wiki
```

Expected acceptable hits:

- This page may mention pattern names for documentation.
- Generic names like `GH_PAT` or `FURY_TOKEN` are acceptable when no secret value is present.

## Current Status

2026-04-22: Initial scan found local absolute paths only. Those were replaced with `<repo-root>`. No secret values were found in the vault seed content.

2026-04-23: A follow-up scan removed remaining machine-specific references from sound and validation notes, added durable banner asset rules, and confirmed local Obsidian workspace state stays ignored instead of entering the public vault.
