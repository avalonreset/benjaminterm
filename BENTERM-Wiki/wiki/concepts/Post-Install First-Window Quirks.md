---
type: concept
title: "Post-Install First-Window Quirks"
created: 2026-04-28
updated: 2026-04-28
status: active
tags:
  - install
  - hyperyap
  - uipi
  - tech-debt
related:
  - "[[v2.0.0 Rebrand Release]]"
  - "[[Release Workflow Tax]]"
  - "[[BENTERM and Hyper Yap Boundary]]"
---

# Post-Install First-Window Quirks

After installing BENTERM via the Inno Setup installer, the FIRST BENTERM window (the one Inno Setup auto-launches at the end of the wizard) has multiple integration failures with HyperYap that disappear the moment the user opens any subsequent BENTERM window from the Start Menu / shortcut. Captured during the v2.0.0 install on 2026-04-28.

This page documents the leading root cause, alternative hypotheses, the diagnostic recipe to confirm next time, and the recommended fixes. Do NOT need a new release just to fix these. Work them into the next install-script touch.

## Symptom

1. User runs `benterm-vX.Y.Z-setup.exe`. Wizard completes with the "Run BENTERM" checkbox enabled (default).
2. Inno Setup's `[Run]` step launches `benterm-gui.exe` from inside the wizard's elevated context.
3. In that first window: HyperYap does NOT work at all. Smart paste, hotkeys, dictation, all dead.
4. User opens any second BENTERM window (Start Menu, taskbar, desktop shortcut, double-clicking the .exe in Explorer).
5. In the second window and every subsequent window: HyperYap works perfectly and stays working.

## Leading Root Cause: UIPI Token Mismatch

Windows User Interface Privilege Isolation (UIPI) blocks lower-integrity processes from sending input or messages to higher-integrity windows. Specifically: a Medium IL process cannot call `SendInput`, `PostMessage`, `SendMessage`, or `SetForegroundWindow` against a High IL window.

**Why the first BENTERM is High IL:**

`ci/windows-installer.iss` `[Run]` section is currently:

```
[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
```

There is NO `runasoriginaluser` flag. By default, Inno Setup spawns `[Run]` programs with the SAME token as the installer process. The installer runs elevated (because it writes to `Program Files` + `HKLM`), so the launched BENTERM inherits that elevated token. Result: post-install BENTERM is High IL.

**Why HyperYap silently fails against High IL BENTERM:**

HyperYap (`hyperyap.exe` + `hyperyap-hotkeys.exe`) runs at Medium IL when launched normally by the user. Its hotkey handler (`hotkeys/src/main.rs:1476` and surrounding code) calls `SendInput`-style APIs against the foreground window. UIPI silently drops those calls when the foreground window is at higher IL than HyperYap. The user sees "nothing happens." HyperYap does not crash, it just no-ops.

**Why subsequent windows work:**

Any BENTERM window launched after the installer wizard exits comes from explorer.exe (Start Menu, desktop shortcut, taskbar, double-click in File Explorer). Those inherit the user's normal Medium IL token. HyperYap and BENTERM are now at the same integrity level, UIPI does not interfere.

## Diagnostic Recipe (Next Time This Happens)

Run while the broken first window IS STILL OPEN. Do not close it before checking.

```powershell
# Check IL of all benterm-gui processes
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class TokCheck {
    [DllImport("advapi32.dll", SetLastError=true)] public static extern bool OpenProcessToken(IntPtr h, uint d, out IntPtr t);
    [DllImport("advapi32.dll", SetLastError=true)] public static extern bool GetTokenInformation(IntPtr t, int c, IntPtr i, uint l, out uint r);
    [DllImport("kernel32.dll", SetLastError=true)] public static extern IntPtr OpenProcess(uint a, bool i, uint p);
    [DllImport("kernel32.dll", SetLastError=true)] public static extern bool CloseHandle(IntPtr h);
    public static int Elev(int pid) {
        IntPtr hProc = OpenProcess(0x1000, false, (uint)pid); if (hProc == IntPtr.Zero) return -1;
        try { IntPtr hTok; if (!OpenProcessToken(hProc, 0x0008, out hTok)) return -2;
            try { IntPtr buf = Marshal.AllocHGlobal(4); try { uint len; if (!GetTokenInformation(hTok, 20, buf, 4, out len)) return -3; return Marshal.ReadInt32(buf); } finally { Marshal.FreeHGlobal(buf); } } finally { CloseHandle(hTok); } } finally { CloseHandle(hProc); }
    }
}
"@
Get-Process benterm-gui, hyperyap, hyperyap-hotkeys -ErrorAction SilentlyContinue | ForEach-Object {
    $e = [TokCheck]::Elev($_.Id)
    [PSCustomObject]@{ PID = $_.Id; Name = $_.ProcessName; Elevated = if ($e -eq 1) {'YES'} elseif ($e -eq 0) {'no'} else {"err $e"} }
} | Format-Table -AutoSize
```

If the first BENTERM shows `Elevated: YES` and HyperYap shows `Elevated: no`, UIPI is confirmed.

## Recommended Fix

Single-line .iss change. In `ci/windows-installer.iss`, find the `[Run]` section and add `runasoriginaluser` to the Flags list:

```
[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent runasoriginaluser
```

`runasoriginaluser` tells Inno Setup to drop back to the user's normal (non-elevated) token before launching the `[Run]` target. The post-install BENTERM then matches every subsequent BENTERM at Medium IL, and HyperYap works from window one.

Land this in the next batch of installer cleanup alongside the [[Release Workflow Tax]] punch list. Does NOT require a v2.0.1 right now: users hit the bug once, then never again because subsequent windows are fine.

## Alternative Hypotheses (Lower Probability)

These are worth ruling out if the diagnostic recipe shows BOTH BENTERMs at Medium IL, which would invalidate the UIPI theory.

### A. Window class registration race

If the first BENTERM creates its window before HyperYap finishes installing its `SetWinEventHook` / global hotkey hooks, HyperYap might miss the initial window and only recognize subsequent windows. Verify by:
- Checking HyperYap process startup time vs. BENTERM startup time (`Get-CimInstance Win32_Process -Filter ...` then compare `CreationDate`).
- If HyperYap was started AFTER the installer completed, this is plausible. If HyperYap was running first, this is ruled out.

### B. HyperYap caches "known terminal HWNDs"

If HyperYap has any in-memory cache of seen window handles and stamps something onto a window when first detected, the first BENTERM window might be added to the cache in a state HyperYap rejects. Source check needed: search `hyperyap/src-tauri/src` and `hyperyap/hotkeys/src` for any HWND cache. If none, this is ruled out.

### C. Foreground focus race

Right after Inno Setup completes, the wizard window may briefly hold focus. The user types into BENTERM thinking it has focus, but events go to the wizard. Subsequent BENTERMs are clearly focused, so things work. Ruled out if the user reports HyperYap stays broken on the first window even after explicitly clicking into it.

## Related Issue: BENTERM v2.0.0 Installs to `Program Files\BenjaminTerm\` Not `Program Files\BENTERM\`

Discovered during the same v2.0.0 ship. Cosmetic: the binary works, the icons work, the registry entries work. Just the folder name does not match the brand.

**What the registry shows:**

`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\` has TWO BENTERM entries pointing at the same folder:

| AppId | DisplayName | InstallLocation |
|-------|-------------|-----------------|
| `{9756A36E-1CE2-4B7A-BBB1-C5F73A625336}_is1` | BenjaminTerm version 2026.02.24 | `C:\Program Files\BenjaminTerm\` |
| `{E79835B5-C418-4C79-BD62-3A18E94B22C3}_is1` | benterm version v2.0.0 | `C:\Program Files\BenjaminTerm\` |

**What happened:**

The legacy BenjaminTerm v1.x installer used AppId `{9756A36E-...}` and DefaultDirName `{autopf}\BenjaminTerm`. That created the dir + uninstaller (`unins000.exe`).

When v2.0.0's installer ran with my new .iss (AppId `{E79835B5-...}`, DefaultDirName `{autopf}\BENTERM`), it should have defaulted to `C:\Program Files\BENTERM\`. Different AppIds, different installs. But the user accepted the wizard's path step (intentionally or by Next-Next-Next), and the install landed in the legacy BenjaminTerm dir, creating a parallel `unins001.exe` next to the existing `unins000.exe`.

Result: two registered installs in Apps & Features, both pointing at the same folder, with v2.0.0 binaries layered on top of the v1.x file tree.

**Why it does not break anything right now:**

The binary at `C:\Program Files\BenjaminTerm\benterm-gui.exe` is the actual v2.0.0 build (file version `20260427-233347-4ccbe04a` matches the v2.0.0 release commit). All the v2.0.0 features ship correctly. The path is just cosmetic.

**Recommended fix for next installer:**

Add to `ci/windows-installer.iss` `[Setup]` block:

```
DisableDirPage=auto
UsePreviousAppDir=no
```

`UsePreviousAppDir=no` forces the installer to honor `DefaultDirName` even if a previous install of the same AppId was at a different path (defensive, not strictly required since AppId is new).

`DisableDirPage=auto` skips the directory-selection page entirely on first install, so the user cannot accidentally route the install elsewhere. Power users can still override with the `/DIR=` command-line switch.

**Cleanup for the current install (optional, not urgent):**

When ready to consolidate:
1. Run `C:\Program Files\BenjaminTerm\unins001.exe` to uninstall the v2.0.0 entry.
2. Run `C:\Program Files\BenjaminTerm\unins000.exe` to uninstall the legacy v1.x entry. Both Apps & Features entries gone, dir cleaned up.
3. Re-run the v2.0.0 installer. With no previous install registered, it defaults to `C:\Program Files\BENTERM\`.
4. (After the .iss fix lands, this will be automatic for fresh installs.)

## Summary Punch List

Two single-line fixes in `ci/windows-installer.iss`, batched into the next installer touch:

1. Add `runasoriginaluser` to the `[Run]` Flags. Fixes the post-install HyperYap dead-on-arrival window.
2. Add `DisableDirPage=auto` and `UsePreviousAppDir=no` under `[Setup]`. Forces fresh installs to land at `Program Files\BENTERM\` regardless of any legacy paths.

Both are low-risk, well-documented Inno Setup features. No code changes outside the .iss.
