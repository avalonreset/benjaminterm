---
type: overview
title: "Architecture Overview"
created: 2026-04-22
updated: 2026-04-22
tags:
  - architecture
  - benjaminterm
status: developing
related:
  - "[[BenjaminTerm]]"
  - "[[Tech Stack]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Architecture Overview

BenjaminTerm is built as a conservative fork of [[WezTerm]]. Most core terminal behavior should remain upstream. BenjaminTerm-specific behavior is layered through configuration, bundled assets, branding, packaging, and narrow source changes for agent attention.

Primary areas:

- [[BenjaminTerm Config Layer]]
- [[Toast Notification Backend]]
- [[GUI Frontend]]
- [[Renderer Attention Pulse]]
- [[Branding And Packaging]]

