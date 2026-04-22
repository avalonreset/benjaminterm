param(
  [string]$WezTermExe = "C:\Program Files\WezTerm\wezterm.exe",
  [string]$WorkingDirectory = "E:\benjaminterm-rebuild-2026-04-21"
)

$ErrorActionPreference = "Stop"

$ConfigFile = Join-Path $WorkingDirectory "extras\benjaminterm\benjaminterm.lua"

if (-not (Test-Path -LiteralPath $WezTermExe)) {
  throw "WezTerm executable not found: $WezTermExe"
}

if (-not (Test-Path -LiteralPath $ConfigFile)) {
  throw "BenjaminTerm preview config not found: $ConfigFile"
}

Start-Process -FilePath $WezTermExe -ArgumentList @(
  "--config-file", $ConfigFile,
  "start",
  "--no-auto-connect",
  "--cwd", $WorkingDirectory,
  "pwsh.exe",
  "-NoLogo"
)
