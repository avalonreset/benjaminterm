---
type: concept
title: "Release Candidate Definition"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - release
  - validation
related:
  - "[[Release Readiness 2026-04-22]]"
  - "[[Release Flow]]"
sources:
  - "[[Project Documentation]]"
---

# Release Candidate Definition

A BenjaminTerm release candidate is ready to push when local validation passes and release workflows are structurally ready.

## Local Release-Candidate Bar

- Rust checks pass for the touched release-critical crates.
- Release shell scripts parse.
- Generated GitHub workflow YAML parses.
- Release docs explain platform status and known caveats.
- GitHub release upload is not blocked by optional package-manager steps.

## Official Release Bar

The project is officially released for a platform only after the tag workflow uploads that platform's artifact and checksum to the GitHub release.
