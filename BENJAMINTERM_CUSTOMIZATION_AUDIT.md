# BenjaminTerm Customization Audit

Compared old BenjaminTerm `fork/main` at `67bfb13da` against clean upstream
WezTerm `origin/main` at `577474d89`.

## Keep

### Theme System

- Preserve a packaged default config layer.
- Preserve the curated pure-black built-in theme pool.
- Preserve duplicate/near-duplicate theme filtering.
- Preserve shuffle-bag randomization so themes do not repeat until the bag is exhausted.
- Preserve manual theme cycling with `Ctrl+Alt+T`.
- Preserve persisted theme state in a user-writable location.

Old implementation:

- `extras/vibe/wezterm.lua`
- `DEFAULT_COLOR_SCHEME = 'Blue Matrix'`
- `pick_scheme_from_shuffle_bag`
- `cycle_theme`

### Default Scale

- Preserve larger startup text.
- Old value was `font_size = 16.0`.
- Preserve `adjust_window_size_when_changing_font_size = false`.

### Bundled Font Strategy

- Do not bundle `OCR A Extended` unless a redistribution license is obtained.
- Use `0xProto` as the BenjaminTerm default.
- Bundle `0xProto` font files and `LICENSE_0XPROTO.txt`.
- Keep upstream-bundled `JetBrains Mono` as the first fallback.
- Keep symbol and emoji fallbacks.

Old implementation used `OCR A Extended`, but the rebuild uses `0xProto`
because it has a stronger hacker-terminal identity and is distributed under
SIL Open Font License 1.1.

### Branding

- Preserve BenjaminTerm app identity.
- Preserve BEN logo/icon.
- Preserve installer name, Start Menu name, executable names, and side-by-side
  install behavior.

Old implementation touched:

- `assets/icon/*`
- `assets/windows/terminal.ico`
- `ci/windows-installer.iss`
- `wezterm-gui/build.rs`
- app id / window class / executable identity helpers

### Windows Toast Click-To-Focus

- Preserve terminal-triggered toast notifications for tools such as Codex that
  emit OSC 9 / OSC 777 notifications.
- Preserve Windows toast click-to-focus so clicking the notification focuses
  the pane/window that raised it.
- Add a brief cyan attention pulse around the activated terminal window so the
  target is visually obvious when multiple terminals are visible.

Rebuild implementation:

- `wezterm-toast-notification/src/lib.rs`
- `wezterm-toast-notification/src/windows.rs`
- `wezterm-gui/src/frontend.rs`
- `wezterm-gui/src/termwindow/mod.rs`
- `wezterm-gui/src/termwindow/render/borders.rs`
- `wezterm-gui/src/scripting/guiwin.rs`
- `wezterm-font/src/lib.rs`

## Probably Keep, But Rebuild Carefully

### Packaged Config Loading

- Old fork changed config discovery from `wezterm.lua` / `.wezterm.lua` to
  executable-specific names like `benjaminterm.lua` / `.benjaminterm.lua`.
- This supports side-by-side WezTerm installs.
- Rebuild only after deciding whether BenjaminTerm should ignore an existing
  user `.wezterm.lua` by default.

### Windows Installer / Portable ZIP Packaging

- Old fork bundled the default config next to the executable.
- Old fork bundled license notices.
- Rebuild packaging from scratch; do not copy old release artifacts or `dist/`.

### Default Shell

- Old config set Windows `default_prog = { 'pwsh.exe', '-NoLogo' }`.
- Decide whether this remains a BenjaminTerm default or should only apply when
  `pwsh.exe` exists.

### Minimal UI Defaults

- Old config disabled tab bar.
- Old config kept normal titlebar by default but supported a borderless toggle.
- These are config-layer customizations and do not require source changes.

## Discuss

### Font Quest

Decision: use `0xProto` as the BenjaminTerm identity font.

`OCR A Extended` is visually on-brand but should not be bundled without an
explicit redistribution license. `0xProto` gives the project a similar
technical personality with clean OFL redistribution terms.

### Font Cycling

- Old config included `Ctrl+Alt+F` font cycling.
- You specifically want the dedicated default font packaged.
- Decide whether cycling is still useful or just noise.

### Borderless Mode

- Old config added `Ctrl+Alt+B` borderless toggle and `Ctrl+Alt+D` drag assist.
- Useful if you still like black-glass mode.
- Not related to Hyper Yap, so this can stay if desired.

### Smart Copy

- Old config made `Ctrl+C` copy selected text, otherwise pass through interrupt.
- This is independent of Hyper Yap.
- Could keep because it solves a common terminal footgun, but it is still a
  keybinding behavior change.

### Clickable File Paths

- Old config added `benpath:` hyperlink handling so local file paths and
  artifact names can be clicked from terminal output.
- Useful for coding workflows.
- Low risk, config-only, but still opinionated.

## Drop For Now

### Hyper Yap Superseded Paste Features

Drop from the rebuild unless explicitly requested later:

- Smart `Ctrl+V` image/text paste.
- Claude/OpenWhispr image-path fallback.
- Clipboard image detection via PowerShell.
- Paste undo/redo.
- `Ctrl+Alt+V` image path fallback.
- Environment flags:
  - `BENJAMINTERM_FORCE_ALT_V_IMAGE_PASTE`
  - `BENJAMINTERM_CLAUDE_IMAGE_PATH_BACKSTOP`
  - `BENJAMINTERM_USE_AT_IMAGE_PATH`

### Resize / ConPTY Internals

Do not carry forward yet:

- `term/src/screen.rs` ConPTY resize changes.
- `wezterm-gui/src/termwindow/resize.rs` render cache flushing.
- related resize tests.

These were likely part of the scrambled behavior risk area.

### Renderer Default Change

- Old config defaulted Windows to `WebGpu`.
- Upstream default should remain unless we confirm the issue still exists.
- Keep only the environment override later if needed.

### Old Release Artifacts

Do not carry forward:

- root `.exe` / `.zip` release artifacts.
- `dist/` release directories.
- generated checksum files.

## Source Change Inventory

High-level old fork changes:

- Config/runtime identity:
  - `config/src/config.rs`
  - `config/src/lib.rs`
  - `config/src/lua.rs`
  - `config/src/unix.rs`
  - `env-bootstrap/src/lib.rs`
  - `env-bootstrap/src/ringlog.rs`
  - `mux/src/domain.rs`
  - `wezterm-client/src/client.rs`
  - `wezterm-mux-server/src/main.rs`
  - `wezterm-mux-server-impl/src/sessionhandler.rs`
  - `wezterm/src/cli/mod.rs`
  - `wezterm-gui-subcommands/src/lib.rs`

- Branding / packaging:
  - `ci/windows-installer.iss`
  - `ci/deploy.sh`
  - `wezterm-gui/build.rs`
  - `wezterm-gui/src/main.rs`
  - `assets/icon/*`
  - `assets/windows/terminal.ico`

- Vibe config:
  - `extras/vibe/wezterm.lua`
  - `extras/vibe/README.md`
  - `extras/vibe/linux/*`

- Risky terminal behavior:
  - `term/src/screen.rs`
  - `term/src/test/mod.rs`
  - `wezterm-gui/src/frontend.rs`
  - `wezterm-gui/src/termwindow/resize.rs`
  - `wezterm-toast-notification/*`

## Recommended Rebuild Order

1. Add a slim `extras/benjaminterm/benjaminterm.lua` with only theme shuffle,
   default font, default scale, tab-bar/UI defaults, and optional borderless
   controls.
2. Set default font to a bundled open font.
3. Add branding assets and Windows installer rename.
4. Add side-by-side runtime identity/config names.
5. Build and run.
6. Review any remaining source-level extras one at a time.
