param(
  [string]$PreviewDirectory = "E:\benjaminterm-rebuild-2026-04-21\preview\benjaminterm-icon-preview",
  [string]$WorkingDirectory = "E:\benjaminterm-rebuild-2026-04-21"
)

$ErrorActionPreference = "Stop"

$Exe = Join-Path $PreviewDirectory "benjaminterm-gui.exe"
$ConfigFile = Join-Path $PreviewDirectory "benjaminterm.lua"

if (-not (Test-Path -LiteralPath $Exe)) {
  throw "Preview executable not found: $Exe"
}

if (-not (Test-Path -LiteralPath $ConfigFile)) {
  throw "Preview config not found: $ConfigFile"
}

Start-Process -FilePath $Exe -ArgumentList @(
  "--config-file", $ConfigFile,
  "start",
  "--no-auto-connect",
  "--cwd", $WorkingDirectory,
  "pwsh.exe",
  "-NoLogo"
)
