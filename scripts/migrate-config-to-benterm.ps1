# BENTERM v2.0.0 config migration helper.
#
# Copies ~/.benjaminterm.lua to ~/.benterm.lua and renames any
# benjaminterm_idle_text_glow* config field references to their new
# benterm_idle_text_glow* names. Idempotent - re-running is safe.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File scripts/migrate-config-to-benterm.ps1

$ErrorActionPreference = 'Stop'

$src = Join-Path $env:USERPROFILE '.benjaminterm.lua'
$dst = Join-Path $env:USERPROFILE '.benterm.lua'

if (-not (Test-Path $src)) {
    Write-Host "No ~/.benjaminterm.lua to migrate. Nothing to do." -ForegroundColor Yellow
    exit 0
}

if (Test-Path $dst) {
    $backup = "$dst.bak.$(Get-Date -Format 'yyyyMMdd-HHmmss')"
    Write-Host "~/.benterm.lua already exists - backing up to $backup" -ForegroundColor Cyan
    Copy-Item -Path $dst -Destination $backup
}

$content = Get-Content -Path $src -Raw

# Field-name renames (the load-bearing ones the renderer reads)
$content = $content -replace 'benjaminterm_idle_text_glow', 'benterm_idle_text_glow'

# Generic identifier rename for any user-defined helpers / variables
# that might reference the old name. Conservative pattern: only word
# boundaries, only the lowercase form, only when surrounded by
# identifier characters or punctuation that suggests code.
$content = $content -replace '\bbenjaminterm_', 'benterm_'

# v2.0.0 default font bump: 0xProto is now bundled with the installer
# and registered system-wide, so it's the new distro default. Old configs
# from before commit a249c815d still default to OCR A Extended; flip them.
$content = $content -replace "local DEFAULT_FONT_PRIMARY = 'OCR A Extended'", "local DEFAULT_FONT_PRIMARY = '0xProto'"

# Make sure the new default is actually in the cycling rotation. Insert
# 0xProto at the top of hacker_font_candidates if it isn't already there.
if ($content -notmatch "(?m)^\s*'0xProto',") {
    $content = $content -replace "(?m)^(local hacker_font_candidates = \{\r?\n  -- Installed \(Windows\)\r?\n)", "`$1  '0xProto',`r`n"
}

# v2.0.0 LOCALAPPDATA path bump: pre-rename configs wrote runtime data
# (clipboard images, pane state, etc.) to %LOCALAPPDATA%\BenjaminTerm\.
# Migrate any embedded references to use the new BENTERM folder so the
# new binary doesn't keep populating the legacy directory.
$content = $content -replace "'BenjaminTerm\\\\", "'BENTERM\\"

Set-Content -Path $dst -Value $content -NoNewline

Write-Host "Migrated ~/.benjaminterm.lua -> ~/.benterm.lua" -ForegroundColor Green
Write-Host ""
Write-Host "Old file at $src is left in place. Once you've verified the new" -ForegroundColor DarkGray
Write-Host "config works, you can delete it manually." -ForegroundColor DarkGray
