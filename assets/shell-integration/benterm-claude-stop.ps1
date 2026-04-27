# BENTERM Claude Code Stop hook helper.
#
# Emits OSC 9 (the agent ready signal) into BENTERM's pty so the
# per-pane attention features (border pulse, cursor-row glow, soft-cue
# sound) fire when Claude Code finishes a turn, the same way they fire
# for Codex (which emits OSC 9 natively).
#
# Why this is harder than it looks: Claude Code spawns its hooks with
# their stdout/stderr redirected to pipes (so Claude can capture hook
# output) AND detached from its conpty. So:
#   - Writing to stdout / stderr → captured by Claude, never reaches
#     BENTERM.
#   - Opening CONOUT$ from the hook subprocess → opens an ephemeral
#     console nobody reads, NOT BENTERM's conpty.
#
# Strategy: walk the parent process chain, FreeConsole from the
# ephemeral console, AttachConsole to a parent (Claude Code → pwsh →
# BENTERM), and THEN open CONOUT$. Whichever ancestor is the
# actual conpty owner gives us the pty BENTERM reads.

$logPath = Join-Path $env:LOCALAPPDATA 'benterm-claude-stop.log'
function _Log($msg) {
    try {
        Add-Content -Path $logPath -Value "[$(Get-Date -Format o)] $msg"
    } catch { }
}

try {
    Add-Type -ErrorAction Stop -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

public static class BTConsole {
    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool FreeConsole();

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool AttachConsole(uint processId);

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern SafeFileHandle CreateFileW(
        string lpFileName,
        uint dwDesiredAccess,
        uint dwShareMode,
        IntPtr lpSecurityAttributes,
        uint dwCreationDisposition,
        uint dwFlagsAndAttributes,
        IntPtr hTemplateFile);
}
"@

    # OSC 9 bytes: ESC ] 9 ; BENTERM Ready BEL
    $bytes = [byte[]]@(
        0x1b, 0x5d, 0x39, 0x3b,
        0x42, 0x45, 0x4e, 0x54, 0x45, 0x52, 0x4d, 0x20,
        0x52, 0x65, 0x61, 0x64, 0x79,
        0x07
    )

    function Try-EmitOnConsoleOf([uint32]$targetPid) {
        # Detach from whatever ephemeral console we're on first.
        [BTConsole]::FreeConsole() | Out-Null
        $attached = [BTConsole]::AttachConsole($targetPid)
        if (-not $attached) {
            $err = [System.Runtime.InteropServices.Marshal]::GetLastWin32Error()
            return @{ ok = $false; err = "AttachConsole($targetPid) failed win32err=$err" }
        }
        try {
            $h = [BTConsole]::CreateFileW(
                'CONOUT$', 0x40000000, 3, [IntPtr]::Zero, 3, 0, [IntPtr]::Zero
            )
            if ($h.IsInvalid) {
                $err = [System.Runtime.InteropServices.Marshal]::GetLastWin32Error()
                return @{ ok = $false; err = "CreateFile(CONOUT$,$targetPid) win32err=$err" }
            }
            try {
                $stream = New-Object System.IO.FileStream(
                    $h, [System.IO.FileAccess]::Write
                )
                try {
                    $stream.Write($script:bytes, 0, $script:bytes.Length)
                    $stream.Flush()
                    return @{ ok = $true; err = $null }
                } finally {
                    $stream.Close()
                }
            } finally {
                $h.Close()
            }
        } finally {
            [BTConsole]::FreeConsole() | Out-Null
        }
    }

    # Walk up the parent chain. Skip bash.exe hook-wrappers (each has
    # its own ephemeral console that AttachConsole succeeds on but
    # writes go nowhere visible). Stop at the first non-wrapper
    # ancestor we can write to — that's the one attached to
    # BENTERM's pty, and writing once avoids the double-sound
    # bug where pwsh AND claude both attach to the same conpty and
    # produce two ready signals per turn.
    $script:bytes = $bytes
    $visited = @()
    $currentPid = $PID
    $hops = 0
    $delivered = $false
    while ($hops -lt 10 -and -not $delivered) {
        $hops++
        try {
            $proc = Get-CimInstance Win32_Process -Filter "ProcessId=$currentPid" -ErrorAction Stop
        } catch {
            _Log "lookup failed for pid=${currentPid}: $($_.Exception.Message)"
            break
        }
        if (-not $proc) { break }
        $parentPid = [uint32]$proc.ParentProcessId
        if ($parentPid -eq 0 -or $visited -contains $parentPid) { break }
        $visited += $parentPid
        $parentName = ''
        try {
            $parentProc = Get-CimInstance Win32_Process -Filter "ProcessId=$parentPid"
            if ($parentProc) { $parentName = $parentProc.Name }
        } catch { }

        # Hook wrappers Claude spawns to run the hook. They sit on
        # their own ephemeral console — never the pty.
        if ($parentName -in @('bash.exe', 'sh.exe', 'dash.exe', 'cmd.exe')) {
            _Log "wrapper-skip pid=$parentPid name=$parentName"
            $currentPid = $parentPid
            continue
        }

        $result = Try-EmitOnConsoleOf $parentPid
        if ($result.ok) {
            _Log "delivered pid=$parentPid name=$parentName"
            $delivered = $true
            break
        } else {
            _Log "skip pid=$parentPid name=$parentName -- $($result.err)"
        }
        $currentPid = $parentPid
    }
    if (-not $delivered) {
        _Log "no delivery (hops=$hops visited=$($visited -join ','))"
    }
} catch {
    _Log "fatal: $($_.Exception.Message)"
}
