---
type: concept
title: "README Banner Asset Workflow"
created: 2026-04-23
updated: 2026-04-23
status: active
tags:
  - readme
  - banner
  - release
  - docs
related:
  - "[[BenjaminTerm]]"
  - "[[Public Release Sanitization]]"
  - "[[Project Documentation]]"
sources:
  - "[[Project Documentation]]"
---

# README Banner Asset Workflow

BenjaminTerm's public GitHub banner is a release-facing asset, not a scratch artifact.

## Canonical Asset

- The live README banner should be stored at `assets/banner.webp`.
- The README should reference that asset directly at the top of the page.
- Keep the banner in `21:9` format so it reads correctly on GitHub.
- Prefer WebP for the committed delivery asset.

## Working Process

- Generate or refine candidate banners under ignored temporary folders such as `tmp/banner-candidates/`.
- Review candidates locally before promoting one into `assets/banner.webp`.
- Only the approved final banner belongs in the tracked asset path.

## Sanitization Rule

- Do not store provider API keys, local output paths, or private environment details in the wiki.
- Do not rely on ephemeral provider result URLs as durable documentation.
- If the generation workflow needs external credentials, describe them generically and keep the values runtime-only.

## Current State

As of 2026-04-23, the public README banner is an approved refined BenjaminTerm WebP variant promoted from the local candidate workflow into `assets/banner.webp`.
