param(
  [string]$BenjaminTermExe = ""
)

$ErrorActionPreference = "Stop"

$Repo = Resolve-Path (Join-Path $PSScriptRoot "..")
$SceneScript = Join-Path $Repo "scripts\screenshot-scene.ps1"

if (-not (Test-Path -LiteralPath $SceneScript)) {
  throw "Screenshot scene script not found: $SceneScript"
}

$PortableExe = Join-Path $Repo "dist\BenjaminTerm-windows-v1.4.1\BenjaminTerm-gui.exe"
$InstalledExe = "C:\Program Files\BenjaminTerm\BenjaminTerm-gui.exe"

if ([string]::IsNullOrWhiteSpace($BenjaminTermExe)) {
  if (Test-Path -LiteralPath $PortableExe) {
    $BenjaminTermExe = $PortableExe
  } else {
    $BenjaminTermExe = $InstalledExe
  }
}

if (-not (Test-Path -LiteralPath $BenjaminTermExe)) {
  throw "BenjaminTerm executable not found. Tried: $BenjaminTermExe"
}

Start-Process -FilePath $BenjaminTermExe -ArgumentList @(
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
