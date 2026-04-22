$ErrorActionPreference = "Stop"

$repo = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$exe = Join-Path $repo "target\debug\BenjaminTerm-gui.exe"
if (!(Test-Path $exe)) {
    throw "Preview executable not found: $exe. Build first with: cargo build -p wezterm-gui"
}

$tmpDir = Join-Path $repo ".tmp"
New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null

$triggerScript = Join-Path $tmpDir "benjaminterm-glow-trigger.ps1"
$trigger = @'
$esc = [char]27
Write-Host "BenjaminTerm glow preview: ready event will fire in 2 seconds."
Start-Sleep -Seconds 2
[Console]::Write("$esc]777;notify;BenjaminTerm;ready$([char]7)")
Write-Host ""
Write-Host "Ready event fired. Watch the cursor row for the breathing glow; typing should clear it."
'@
Set-Content -LiteralPath $triggerScript -Value $trigger -Encoding UTF8

Start-Process -FilePath $exe -ArgumentList @(
    "start",
    "powershell.exe",
    "-NoExit",
    "-ExecutionPolicy",
    "Bypass",
    "-File",
    $triggerScript
) -WorkingDirectory $repo
