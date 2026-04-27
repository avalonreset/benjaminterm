param(
  [string]$PreviewDirectory = "E:\benterm-rebuild-2026-04-21\preview\benterm-icon-preview",
  [string]$WorkingDirectory = "E:\benterm-rebuild-2026-04-21"
)

$ErrorActionPreference = "Stop"

$Exe = Join-Path $PreviewDirectory "benterm-gui.exe"
$ConfigFile = Join-Path $PreviewDirectory "benterm.lua"

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
