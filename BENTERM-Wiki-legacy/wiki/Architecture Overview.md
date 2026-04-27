---
type: overview
title: "Architecture Overview"
created: 2026-04-22
updated: 2026-04-22
tags:
  - architecture
  - benterm
status: developing
related:
  - "[[BENTERM]]"
  - "[[Tech Stack]]"
sources:
  - "[[Rebuild Session Summary]]"
---

# Architecture Overview

BENTERM is built as a conservative fork of [[WezTerm]]. Most core terminal behavior should remain upstream. BENTERM-specific behavior is layered through configuration, bundled assets, branding, packaging, and narrow source changes for agent attention.

Primary areas:

- [[BENTERM Config Layer]]
- [[Toast Notification Backend]]
- [[GUI Frontend]]
- [[Renderer Attention Pulse]]
- [[Branding And Packaging]]

