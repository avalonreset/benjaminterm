param(
  [string]$WezTermExe = "C:\Program Files\WezTerm\wezterm.exe",
  [string]$WorkingDirectory = "E:\benterm-rebuild-2026-04-21"
)

$ErrorActionPreference = "Stop"

$ConfigFile = Join-Path $WorkingDirectory "extras\benterm\benterm.lua"

if (-not (Test-Path -LiteralPath $WezTermExe)) {
  throw "WezTerm executable not found: $WezTermExe"
}

if (-not (Test-Path -LiteralPath $ConfigFile)) {
  throw "BENTERM preview config not found: $ConfigFile"
}

Start-Process -FilePath $WezTermExe -ArgumentList @(
  "--config-file", $ConfigFile,
  "start",
  "--no-auto-connect",
  "--cwd", $WorkingDirectory,
  "pwsh.exe",
  "-NoLogo"
)
