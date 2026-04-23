---
type: concept
title: "Cross-Platform Release Strategy"
created: 2026-04-22
updated: 2026-04-22
status: active
tags:
  - release
  - cross-platform
related:
  - "[[Release Flow]]"
  - "[[Packaging and CI]]"
  - "[[Release Readiness 2026-04-22]]"
sources:
  - "[[Project Documentation]]"
---

# Cross-Platform Release Strategy

BenjaminTerm should be positioned as a cross-platform terminal distribution, not as a Windows-only fork.

## Practical Meaning

- Windows, Linux, and macOS are first-class release targets.
- GitHub release artifacts are the official distribution baseline.
- Package-manager ecosystems are useful but optional.
- The release is official only after tag workflows attach platform artifacts and checksums.

## Related Notes

- [[Release Flow]]
- [[Packaging and CI]]
- [[Open Release Questions]]
