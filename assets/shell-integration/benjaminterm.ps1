# BenjaminTerm pwsh shell integration.
#
# Emits OSC 9 (the agent ready signal per BenjaminTerm Suite Mandatory
# Requirements M1) after each command. OSC 9 is what Codex emits
# natively to fire the per-pane attention features (border pulse,
# cursor-row glow, soft-cue sound). Sourcing this from $PROFILE makes
# plain pwsh sessions behave the same way without any per-pane wiring.
#
# Also emits OSC 133 (FinalTerm) markers so BenjaminTerm can track
# prompt boundaries for paste-undo and clickable path resolution.
#
# Source from your $PROFILE:
#     . "C:\Program Files\BenjaminTerm\shell-integration\benjaminterm.ps1"
#
# Or wire it into default_prog in your benjaminterm.lua:
#     config.default_prog = {
#       'pwsh.exe', '-NoLogo', '-NoExit',
#       '-Command',
#       '. "C:/Program Files/BenjaminTerm/shell-integration/benjaminterm.ps1"'
#     }

$global:__BTPromptCount = 0
$__BTOriginalPrompt = $function:prompt

function global:prompt {
    if ($global:__BTPromptCount -gt 0) {
        # OSC 133;D — FinalTerm "command done" marker
        [Console]::Write("$([char]27)]133;D$([char]7)")
        # OSC 9 — agent ready signal. Fires per-pane attention pulse,
        # cursor-row glow, and soft-cue sound in BenjaminTerm.
        [Console]::Write("$([char]27)]9;BenjaminTerm Ready$([char]7)")
    }
    $global:__BTPromptCount++

    # OSC 133;A — FinalTerm "prompt start" marker
    [Console]::Write("$([char]27)]133;A$([char]7)")

    $promptText = & $__BTOriginalPrompt
    if (-not $promptText) {
        $promptText = "PS $($PWD.Path)> "
    }

    # OSC 133;B — FinalTerm "command start" marker
    [Console]::Write("$([char]27)]133;B$([char]7)")
    $promptText
}
