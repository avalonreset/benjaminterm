$ErrorActionPreference = "Stop"

$esc = [char]27
$osc = "$esc]"
$bel = [char]7

function Write-Line {
  param(
    [string]$Text = "",
    [string]$Color = "Cyan"
  )
  Write-Host $Text -ForegroundColor $Color
}

function Write-Muted {
  param([string]$Text = "")
  Write-Host $Text -ForegroundColor DarkCyan
}

function Write-Accent {
  param([string]$Text = "")
  Write-Host $Text -ForegroundColor Cyan
}

function Write-Rule {
  Write-Host "===============================================================================" -ForegroundColor DarkCyan
}

Clear-Host
[Console]::Title = "BenjaminTerm"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()

Write-Host "${osc}8;;https://github.com/avalonreset/BenjaminTerm${bel}BenjaminTerm${osc}8;;${bel}" -NoNewline -ForegroundColor Red
Write-Host "  //  AI CODING TERMINAL  //  WINDOWS  MACOS  LINUX" -ForegroundColor Cyan
Write-Rule
Write-Host ""

Write-Accent "   ___  _____  __   _____   __  ________  _______________  __  ___"
Write-Accent "  / _ )/ __/ |/ /_ / / _ | /  |/  /  _/ |/ /_  __/ __/ _ \/  |/  /"
Write-Accent " / _  / _//    / // / __ |/ /|_/ // //    / / / / _// , _/ /|_/ /"
Write-Accent "/____/___/_/|_/\___/_/ |_/_/  /_/___/_/|_/ /_/ /___/_/|_/_/  /_/"
Write-Host ""

Write-Line "Release: v1.4.1" "White"
Write-Muted "Official artifacts: Windows installer + portable zip, macOS app zip, Linux tarball"
Write-Muted "Built on WezTerm. Branded for BenjaminTerm. Tuned for long AI coding sessions."
Write-Host ""

Write-Rule
Write-Line "WHAT BENJAMINTERM DOES" "White"
Write-Host ""
Write-Accent "  [attention]  Soft per-pane sound identity when an agent is ready"
Write-Accent "  [visual]     Theme-aware ready pulse and background tab marker"
Write-Accent "  [focus]      Windows reminder toasts that jump back to the exact session"
Write-Accent "  [font]       0xProto bundled, no font scavenger hunt"
Write-Accent "  [release]    Same GitHub release path for Windows, macOS, and Linux"
Write-Host ""

Write-Rule
Write-Line "WHERE HYPERYAP FITS" "White"
Write-Host ""
Write-Host "${osc}8;;https://github.com/avalonreset/hyperyap${bel}HyperYap${osc}8;;${bel}" -NoNewline -ForegroundColor Cyan
Write-Host " handles voice, paste, screenshots, image routing, and app-wide hotkeys." -ForegroundColor DarkCyan
Write-Muted "BenjaminTerm stays focused on the terminal surface: readable output, session identity,"
Write-Muted "agent-ready attention, and a clean place for Codex or Claude to work."
Write-Host ""

Write-Rule
Write-Line "THE LOOP" "White"
Write-Host ""
Write-Accent "  1. Start an agent in BenjaminTerm"
Write-Accent "  2. Let the window fade into the background"
Write-Accent "  3. Hear the pane-specific cue when work finishes"
Write-Accent "  4. See the ready pulse or tab marker"
Write-Accent "  5. Jump straight back into the right session"
Write-Host ""

Write-Muted "No package-manager maze. No missing-font nags. No guessing which terminal is ready."
Write-Host ""
Write-Rule
Write-Host "https://github.com/avalonreset/BenjaminTerm" -ForegroundColor DarkCyan
Write-Host "https://github.com/avalonreset/hyperyap" -ForegroundColor DarkCyan
Write-Host ""

Write-Host "> " -NoNewline -ForegroundColor Cyan
Write-Host "ready for the next prompt" -ForegroundColor White
