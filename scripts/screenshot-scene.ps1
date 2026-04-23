param(
  [int]$Columns = 104,
  [int]$Rows = 34,
  [switch]$NoHold
)

$ErrorActionPreference = "Stop"

$esc = [char]27
$reset = "$esc[0m"
$bold = "$esc[1m"
$dim = "$esc[38;2;82;111;132m"
$cyan = "$esc[38;2;80;220;255m"
$blue = "$esc[38;2;80;150;255m"
$red = "$esc[38;2;255;68;92m"
$white = "$esc[38;2;235;245;255m"
$green = "$esc[38;2;80;255;170m"
$violet = "$esc[38;2;190;120;255m"
$gold = "$esc[38;2;255;205;90m"

function Out-Raw {
  param([string]$Text = "")
  [Console]::Write($Text)
}

function Line {
  param(
    [string]$Text = "",
    [string]$Color = $white
  )
  Out-Raw "$Color$Text$reset`n"
}

function Blank {
  Out-Raw "`n"
}

try {
  & mode.com con: "cols=$Columns" "lines=$Rows" | Out-Null
} catch {
  # Some terminal hosts ignore console resize requests. The scene still prints cleanly.
}

Clear-Host
[Console]::Title = "BenjaminTerm - Screenshot Scene"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()

Line "  BEN  ${bold}BENJAMINTERM$reset$cyan  v1.4.1  $dim//  AI coding terminal for Windows, macOS, and Linux" $red
Line "  ================================================================================================" $blue
Blank

Line "   ___  _____  __   _____   __  ________  _______________  __  ___" $cyan
Line "  / _ )/ __/ |/ /_ / / _ | /  |/  /  _/ |/ /_  __/ __/ _ \/  |/  /" $cyan
Line " / _  / _//    / // / __ |/ /|_/ // //    / / / / _// , _/ /|_/ /" $cyan
Line "/____/___/_/|_/\___/_/ |_/_/  /_/___/_/|_/ /_/ /___/_/|_/_/  /_/" $cyan
Blank

Line "  $bold$white Built from WezTerm. Styled for long agent sessions. Packaged for all three desktops.$reset" $white
Blank

Line "  $blue+-- AGENT READY ------------------------------+   +-- RELEASE CHANNEL --------------------+" $blue
Line "  $blue|$reset $green*$reset Codex finished in pane 3                    $blue|$reset   $green*$reset Windows installer + portable zip     $blue|" $white
Line "  $blue|$reset $cyan*$reset Soft cue assigned to this pane               $blue|$reset   $green*$reset macOS app zip                         $blue|" $white
Line "  $blue|$reset $violet*$reset Theme-aware pulse marks the right window    $blue|$reset   $green*$reset Linux tarball                         $blue|" $white
Line "  $blue|$reset $gold*$reset Toast click jumps back to the exact session   $blue|$reset   $green*$reset Checksums attached                     $blue|" $white
Line "  $blue+---------------------------------------------+   +---------------------------------------+" $blue
Blank

Line "  $red+-- BENJAMINTERM OWNS -------------------------+   +-- HYPERYAP OWNS ----------------------+" $red
Line "  $red|$reset Terminal identity and release packaging           $red|$reset   Voice to text and dictation             $red|" $white
Line "  $red|$reset 0xProto bundled as the default coding font        $red|$reset   Smart paste and image routing           $red|" $white
Line "  $red|$reset Per-pane sound identity for agent completion      $red|$reset   App-wide hotkeys and capture workflow    $red|" $white
Line "  $red|$reset Visual ready pulse and background tab marker      $red|$reset   The full workstation layer              $red|" $white
Line "  $red+---------------------------------------------+   +---------------------------------------+" $red
Blank

Line "  $dim No package-manager maze. No missing-font nags. No guessing which terminal is ready.$reset" $dim
Line "  $dim Official download: https://github.com/avalonreset/BenjaminTerm/releases/latest$reset" $dim
Line "  $dim Companion project: https://github.com/avalonreset/hyperyap$reset" $dim
Blank

Line "  $cyan>$reset $white waiting for the next prompt $green#$reset" $white

if (-not $NoHold -and $env:BENJAMINTERM_SCREENSHOT_NO_HOLD -ne "1") {
  while ($true) {
    Start-Sleep -Seconds 3600
  }
}
