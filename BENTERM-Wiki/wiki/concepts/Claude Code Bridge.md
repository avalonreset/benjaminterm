---
type: concept
title: "Claude Code Bridge"
created: 2026-04-27
updated: 2026-04-27
status: active
tags:
  - integration
  - claude-code
  - hooks
  - osc
  - windows
related:
  - "[[Attention Trigger Lifecycle]]"
  - "[[Sound Grab Bag Attention System]]"
sources:
  - "[[Project Documentation]]"
---

# Claude Code Bridge

Claude Code does not emit `OSC 9` (the BENTERM agent ready signal per Mandatory Requirements M1) when it finishes a turn. To make Claude trigger the per-pane attention features (border pulse, cursor-row glow, soft-cue sound) the same way Codex does natively, the BENTERM install ships a **Claude Code `Stop` hook helper** at `assets/shell-integration/benterm-claude-stop.ps1`.

## Why a hook helper is needed

A naive Stop hook that just runs `printf '\033]9;BENTERM Ready\007' > /dev/tty` does **not** work on Windows. Two problems:

1. **stdout / stderr are pipes.** Claude Code spawns its hook subprocesses with their stdio redirected to pipes so it can capture hook output for the user-facing transcript. Anything written to stdout / stderr is consumed by Claude, never passes through to the terminal as raw bytes.
2. **`CONOUT$` opens an ephemeral console.** Claude Code spawns hooks with their console attachment detached from BENTERM's conpty. The hook subprocess gets its own private console nobody reads. `[IO.File]::OpenWrite('CONOUT$')` even fails outright because `CONOUT$` is a device, not a file - it requires `CreateFile` + a `FileStream` constructor that takes the OS handle.

The helper handles both with a parent-walk + reattach.

## The parent-walk technique

```
1. Walk up the process chain via Win32_Process ParentProcessId.
2. Skip wrapper shells (bash.exe, sh.exe, dash.exe, cmd.exe) - they
   have their own ephemeral consoles and AttachConsole succeeds on
   them but the bytes go nowhere.
3. For each non-wrapper ancestor:
     FreeConsole()             // detach from current ephemeral console
     AttachConsole(parent_pid) // attach to ancestor's console
     CreateFile("CONOUT$", ...) // open the now-correct console handle
     FileStream.Write(osc9)
     FreeConsole()
4. Stop on first successful write. The first non-wrapper ancestor that
   has a console is the conpty owner BENTERM reads.
```

The "stop on first success" rule is load-bearing: pwsh AND claude.exe are both attached to the same conpty (one inherited from the other), so writing to both produces two OSC 9 events and two sounds per ready event.

## Wiring the hook

In `~/.claude/settings.json`:

```json
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "powershell -NoProfile -ExecutionPolicy Bypass -File <install-dir>/shell-integration/benterm-claude-stop.ps1"
          }
        ]
      }
    ]
  }
}
```

Use forward slashes in the `-File` path. Bash (which Claude uses to run the hook command when `CLAUDE_CODE_GIT_BASH_PATH` is set) treats backslash + letter as an escape and silently mangles the path - `<repo-root>\subdir\...` becomes `<repo-root>subdir...` and `iscc` errors with "file does not exist."

## Diagnostics

The helper appends every attempt to `%LOCALAPPDATA%\benterm-claude-stop.log` so you can see exactly which ancestor was hit and why. Each Stop hook fire produces one line like:

```
[2026-04-27T19:08:19] delivered pid=12345 name=claude.exe
```

If you see `wrapper-skip` lines for every ancestor and no `delivered`, the conpty owner is unreachable from the hook subprocess - you're in an environment where the Claude Code hook execution model has changed. Check the bundled helper's parent-walk loop against the current Claude Code release.

## What this does NOT solve

- Claude Code TUI repaints on resize. Conpty + alternate-screen-buffer + resize on Windows is a known cluster of upstream WezTerm + Windows conpty quirks. The attention features fire correctly but visible artifacts in the agent's UI on resize are not in scope for this bridge.
- Other agent CLIs. The same parent-walk pattern would work for any agent CLI that runs a hook subprocess but doesn't emit OSC 9 itself - the helper script would just need a per-agent variant pointing at that CLI's hook system. Codex needs nothing because it emits OSC 9 natively.
