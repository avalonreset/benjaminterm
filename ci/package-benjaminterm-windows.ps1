param(
  [string]$TargetDir = "target",
  [string]$Configuration = "release",
  [string]$TagName = "",
  [string]$DistDir = "dist"
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($TagName)) {
  $TagName = git -c "core.abbrev=8" show -s "--format=%cd-%h" "--date=format:%Y%m%d-%H%M%S"
}

$releaseDir = Join-Path $TargetDir $Configuration
$packageName = "BenjaminTerm-windows-$TagName"
$packageDir = Join-Path $DistDir $packageName
$zipPath = Join-Path $DistDir "$packageName.zip"

if (Test-Path $packageDir) {
  Remove-Item -LiteralPath $packageDir -Recurse -Force
}
if (Test-Path $zipPath) {
  Remove-Item -LiteralPath $zipPath -Force
}

New-Item -ItemType Directory -Force -Path $packageDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $packageDir "mesa") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $packageDir "fonts") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $packageDir "sounds") | Out-Null

$requiredFiles = @(
  @{ Source = "wezterm.exe"; Destination = "BenjaminTerm.exe" },
  @{ Source = "wezterm-gui.exe"; Destination = "BenjaminTerm-gui.exe" },
  @{ Source = "wezterm-mux-server.exe"; Destination = "BenjaminTerm-mux-server.exe" },
  @{ Source = "strip-ansi-escapes.exe"; Destination = "strip-ansi-escapes.exe" }
)

foreach ($file in $requiredFiles) {
  $source = Join-Path $releaseDir $file.Source
  if (!(Test-Path $source)) {
    throw "Missing required build output: $source"
  }
  Copy-Item -LiteralPath $source -Destination (Join-Path $packageDir $file.Destination)
}

$optionalPdb = Join-Path $releaseDir "wezterm.pdb"
if (Test-Path $optionalPdb) {
  Copy-Item -LiteralPath $optionalPdb -Destination $packageDir
}

Copy-Item -LiteralPath "assets\windows\conhost\conpty.dll" -Destination $packageDir
Copy-Item -LiteralPath "assets\windows\conhost\OpenConsole.exe" -Destination $packageDir
Copy-Item -LiteralPath "assets\windows\angle\libEGL.dll" -Destination $packageDir
Copy-Item -LiteralPath "assets\windows\angle\libGLESv2.dll" -Destination $packageDir

$mesaFromBuild = Join-Path $releaseDir "mesa\opengl32.dll"
if (Test-Path $mesaFromBuild) {
  Copy-Item -LiteralPath $mesaFromBuild -Destination (Join-Path $packageDir "mesa")
} else {
  Copy-Item -LiteralPath "assets\windows\mesa\opengl32.dll" -Destination (Join-Path $packageDir "mesa")
}

Copy-Item -LiteralPath "extras\benjaminterm\benjaminterm.lua" -Destination (Join-Path $packageDir "wezterm.lua")
Copy-Item -Path "assets\fonts\*" -Destination (Join-Path $packageDir "fonts") -Recurse
Copy-Item -LiteralPath "assets\sounds\benjaminterm-soft-cues" -Destination (Join-Path $packageDir "sounds") -Recurse

Compress-Archive -Path $packageDir -DestinationPath $zipPath -Force

Write-Host "Packaged $zipPath"
