# BenjaminTerm Rebuild Plan

Baseline: WezTerm upstream `origin/main` at `577474d89` on 2026-04-21.

Current rebuild branch: `benjaminterm-rebuild-2026-04-21`.

## Ground Rules

- Start from clean upstream WezTerm and apply BenjaminTerm changes in small, reviewable commits.
- Keep release artifacts out of source commits until a release build is intentionally produced.
- Prefer additive defaults, docs, assets, and packaging changes before touching terminal core behavior.
- Treat terminal resize, conpty, mux, screen, and renderer behavior as high-risk areas requiring focused tests.
- Keep WezTerm remotes separate: `origin` is upstream WezTerm, `fork` is BenjaminTerm.

## Candidate Feature Groups

1. Branding
   - BenjaminTerm name and runtime identity.
   - BenjaminTerm icons and Windows installer branding.
   - Side-by-side install behavior with upstream WezTerm.

2. Vibe defaults
   - Default config profile.
   - Theme and font cycling.
   - Shuffle-bag theme rotation.
   - Cross-platform config state location.
   - Bundled `0xProto` default font with `JetBrains Mono` fallback.

3. Notifications
   - Preserve terminal-triggered toast notifications.
   - Preserve Windows toast click-to-focus for the originating pane/window.
   - Add a short visual attention pulse after toast activation.

4. Docs and packaging
   - README and release notes.
   - Linux bootstrap docs/scripts.
   - Third-party notices and installer license bundle.
   - Include `assets/fonts/LICENSE_0XPROTO.txt` in binary distributions.

## Hold Until Explicitly Chosen

- ConPTY resize and terminal reflow changes.
- Renderer default changes.
- Smart paste, image paste, OpenWhispr/Claude paste helpers, and paste undo/redo.
- Large `dist/` directories and generated `.zip` / `.exe` artifacts.
- Any broad crate-level rename that affects IPC, config paths, or update checks.

## Old Fork Reference

Old BenjaminTerm branch: `fork/main` at `67bfb13da`.

Shared old upstream base: `05343b387`.

The old fork has 60 commits beyond that base. Reapply only the selected feature groups.
