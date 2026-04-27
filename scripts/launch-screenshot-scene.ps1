param(
  [string]$BENTERMExe = "",
  [int]$Columns = 104,
  [int]$Rows = 34,
  [double]$FontSize = 16.0
)

$ErrorActionPreference = "Stop"

$Repo = Resolve-Path (Join-Path $PSScriptRoot "..")
$SceneScript = Join-Path $Repo "scripts\screenshot-scene.ps1"
$TmpDir = Join-Path $Repo ".tmp\screenshot-scene"
$ConfigFile = Join-Path $TmpDir "benterm-screenshot.lua"

if (-not (Test-Path -LiteralPath $SceneScript)) {
  throw "Screenshot scene script not found: $SceneScript"
}

$PortableExe = Join-Path $Repo "dist\BENTERM-windows-v1.4.1\BENTERM-gui.exe"
$InstalledExe = "C:\Program Files\BENTERM\BENTERM-gui.exe"

if ([string]::IsNullOrWhiteSpace($BENTERMExe)) {
  if (Test-Path -LiteralPath $PortableExe) {
    $BENTERMExe = $PortableExe
  } else {
    $BENTERMExe = $InstalledExe
  }
}

if (-not (Test-Path -LiteralPath $BENTERMExe)) {
  throw "BENTERM executable not found. Tried: $BENTERMExe"
}

New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

$luaFontSize = $FontSize.ToString([System.Globalization.CultureInfo]::InvariantCulture)
$config = @"
local wezterm = require 'wezterm'

return {
  automatically_reload_config = false,
  check_for_updates = false,
  warn_about_missing_glyphs = false,

  initial_cols = $Columns,
  initial_rows = $Rows,
  font_size = $luaFontSize,
  font = wezterm.font_with_fallback({
    '0xProto',
    'Cascadia Mono',
    'JetBrains Mono',
    'Consolas',
  }),

  enable_tab_bar = false,
  hide_tab_bar_if_only_one_tab = true,
  window_decorations = 'RESIZE',
  window_close_confirmation = 'NeverPrompt',
  window_padding = {
    left = 24,
    right = 24,
    top = 20,
    bottom = 18,
  },

  colors = {
    foreground = '#d9f7ff',
    background = '#020407',
    cursor_bg = '#50ffaa',
    cursor_border = '#50ffaa',
    cursor_fg = '#020407',
    selection_bg = '#12364a',
    selection_fg = '#ffffff',
  },

  audible_bell = 'Disabled',
}
"@

Set-Content -LiteralPath $ConfigFile -Value $config -Encoding UTF8

Start-Process -FilePath $BENTERMExe -ArgumentList @(
  "--config-file",
  $ConfigFile,
  "start",
  "--cwd",
  $Repo,
  "powershell.exe",
  "-NoExit",
  "-ExecutionPolicy",
  "Bypass",
  "-File",
  $SceneScript
)
