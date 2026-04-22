# BenjaminTerm

BenjaminTerm is a Windows-first terminal for AI coding sessions, built from a fresh WezTerm baseline and tuned to work cleanly beside HyperYap.

[![Release](https://img.shields.io/github/v/release/avalonreset/BenjaminTerm?include_prereleases&label=release)](https://github.com/avalonreset/BenjaminTerm/releases)
[![Windows](https://img.shields.io/badge/windows-primary-1f6feb)](https://github.com/avalonreset/BenjaminTerm/releases)
[![macOS and Linux](https://img.shields.io/badge/macOS%20%2B%20Linux-best%20effort-555)](https://github.com/avalonreset/BenjaminTerm/releases)
[![Built from WezTerm](https://img.shields.io/badge/built%20from-WezTerm-7c3aed)](https://wezterm.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.md)

## What It Is

BenjaminTerm keeps the stability and terminal depth of WezTerm, then adds a focused set of defaults for running tools like Codex and Claude:

- Windows toast notifications when an agent session is ready for the next prompt.
- Toast click-to-focus so the right terminal comes forward.
- Visual attention pulse on completion, including a tab marker when the ready pane is in the background.
- Per-pane sound grab bag using bundled CC0 interface sounds.
- Bundled 0xProto font so new machines do not need manual font setup.
- Theme shuffle-bag so each new window gets a fresh curated look.
- Fancy tab bar behavior that avoids wasting vertical space when only one tab is open.
- Preset font scale and terminal defaults for a more useful first launch.

HyperYap handles the speech-to-text and clipboard workflow. BenjaminTerm does not try to duplicate that layer.

## Download

Latest preview release:

https://github.com/avalonreset/BenjaminTerm/releases/tag/benjaminterm-v2026.04.22

Recommended Windows installer:

`BenjaminTerm-benjaminterm-v2026.04.22-setup.exe`

Portable Windows zip:

`BenjaminTerm-windows-benjaminterm-v2026.04.22.zip`

Best-effort builds:

- macOS: `BenjaminTerm-macos-benjaminterm-v2026.04.22.zip`
- Linux: `BenjaminTerm-linux-benjaminterm-v2026.04.22.tar.gz`

Windows is the supported target for this preview. macOS and Linux artifacts build successfully in GitHub Actions, but they have not been manually tested yet.

## Install On Windows

1. Download the setup executable from the latest release.
2. Run the installer.
3. Launch Benjamin Term from the Start menu.

The installer uses its own BenjaminTerm Windows app identity and does not add `wezterm.exe` to PATH. That keeps it from casually shadowing vanilla WezTerm from a command prompt.

## Using The Attention System

The completion attention system is designed for a workflow with several terminal windows or tabs open at once.

- When an agent session is ready, BenjaminTerm plays the pane's assigned sound.
- The terminal also shows a visual pulse so your eyes can find the ready pane.
- If the ready pane is not focused, Windows shows a reminder toast.
- Clicking the toast focuses the matching window and triggers another visual cue.
- Typing into the pane clears the pending reminder.

This is intentionally Pavlovian: each pane gets its own subtle sound, so your brain can start mapping tasks to audio cues.

## Attention Philosophy

BenjaminTerm treats agent completion as an attention routing problem.

The normal terminal model assumes the user is staring at one shell. AI coding does not work that way. A real workflow often has multiple windows, multiple tabs, and multiple agents moving at different speeds. When one of them finishes, the terminal should not merely change a taskbar shade and hope the user notices. It should point attention to the right place without becoming loud or annoying.

BenjaminTerm uses a layered strategy:

- Sound first: each new pane gets a sound from a shuffled grab bag, so different work streams begin to feel distinct.
- Color and light second: the ready terminal gets a visual pulse using theme-aware highlight colors.
- Tabs third: if the ready pane is hidden behind another tab, the tab gets an attention marker.
- Toasts only when useful: Windows reminder toasts appear when the ready pane is not focused.
- Click-to-focus as confirmation: clicking a toast brings the matching window forward and triggers a second visual cue.
- Input clears intent: when the user starts typing into the pane, the pending reminder is cleared.

The point is not novelty. The point is reducing cognitive load. The sound tells you something finished. The color tells you where it finished. The toast helps recover buried windows. The click confirms the target. Together, those cues create a faster loop for working across several AI sessions without constantly scanning the taskbar.

The sound grab bag is deliberately randomized. If you open several terminals or split work across tabs, BenjaminTerm assigns cues from the bundle so the sessions do not all sound identical. Over time, that gives each active work stream a lightweight identity.

## Themes And Font

BenjaminTerm ships with a curated theme grab bag and one selected bundled font.

- Primary font: 0xProto
- Sounds: Kenney Interface Sounds, distributed as CC0

The theme shuffle state is stored in the BenjaminTerm state file so new windows cycle through the bag instead of constantly repeating the same theme.

## Font Philosophy

The 0xProto decision is one of the core BenjaminTerm product decisions.

The thesis is simple: a terminal is not just a shell. It is a reading instrument. If you spend hours scanning diffs, prompts, logs, stack traces, generated code, and command output, the font is part of the productivity system. A better programming font can reduce ambiguity, lower eye fatigue, and make long AI coding sessions feel less chaotic.

0xProto was selected because it is built around source code legibility. The project emphasizes clear differentiation between similar-looking characters, readability at small sizes, more even visual balance in monospace text, and ligatures that do not mutate the original meaning of code tokens.

That lines up with BenjaminTerm's philosophy:

- Ambiguous characters slow people down.
- Small terminal text should still have open, readable interiors.
- Dense code should feel balanced, not blotchy.
- Ligatures should help recognition without hiding the actual source text.
- The best default is the one users do not have to think about.

This matters even more when working with agents. AI coding creates fast-moving walls of text: plans, patches, shell output, test logs, and errors. In that environment, typography becomes part of the control surface. BenjaminTerm chooses 0xProto so every install starts with a font that is sharp, hackerish, legally bundleable, and optimized for the kind of reading developers actually do.

0xProto source and license:

https://github.com/0xType/0xProto

## Relationship To WezTerm

BenjaminTerm is a fork of WezTerm. The terminal core, GPU renderer, multiplexer, configuration model, and much of the release machinery come from WezTerm.

Upstream project:

https://github.com/wezterm/wezterm

Documentation for core WezTerm behavior:

https://wezterm.org/

BenjaminTerm changes are intentionally scoped. The goal is to add a productive AI coding layer without rewriting the terminal engine.

## Build From Source

Windows release build:

```powershell
cargo build -p wezterm -p wezterm-gui -p wezterm-mux-server -p strip-ansi-escapes --release
```

Create a local Windows portable zip:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\ci\package-benjaminterm-windows.ps1 -TagName local
```

The tag-driven release workflow lives at:

`.github/workflows/benjaminterm-release.yml`

Tags matching `benjaminterm-v*` build Windows, macOS, and Linux artifacts.

## Current Status

BenjaminTerm `v2026.04.22` is a preview release.

- Windows installer: built and published.
- Windows portable zip: built and published.
- macOS zip: built and published, best effort.
- Linux tarball: built and published, best effort.
- Windows attention behavior: working in local testing.
- macOS/Linux notification behavior: planned best-effort follow-up.

## License

BenjaminTerm keeps WezTerm's MIT license. The bundled 0xProto font and bundled sounds retain their own licenses. See [LICENSE.md](LICENSE.md), `assets/fonts`, and `assets/sounds/kenney-interface`.
