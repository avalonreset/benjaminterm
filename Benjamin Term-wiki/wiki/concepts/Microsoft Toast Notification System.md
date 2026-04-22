---
type: concept
title: "Microsoft Toast Notification System"
created: 2026-04-21
updated: 2026-04-21
tags:
  - windows
  - notifications
  - research
status: developing
validation: "cargo check -p wezterm-gui; git diff --check; wiki link lint"
related:
  - "[[Windows Toast Click-To-Focus]]"
  - "[[Toast Notification Backend]]"
  - "[[Agent Completion Attention Flow]]"
sources:
  - "https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-toast"
  - "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-quickstart"
  - "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-progress-bar"
  - "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-headers"
  - "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-ux-guidance"
  - "https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-content"
---

# Microsoft Toast Notification System

Microsoft is renaming toast notifications to app notifications, but the XML and WinRT concepts still use toast names in many places.

## Core Model

- A toast/app notification appears as a transient popup and is also stored in Notification Center.
- The XML root is `<toast>`. Important attributes for BenjaminTerm are `launch`, `duration`, `scenario`, and `displayTimestamp`.
- `launch` carries activation arguments. BenjaminTerm uses this for `focus-pane:<pane_id>`.
- Standard `duration` only supports `short` or `long`. It does not mean indefinite.
- Local app notifications have a default and maximum Notification Center expiration of three days unless explicitly cleared earlier.

## Scenarios

The `scenario` attribute changes how Windows treats the notification:

- `default`: normal transient notification behavior.
- `reminder`: pre-expanded and intended to stay on screen until dismissed, but Microsoft notes it can be ignored unless the toast has a button action that activates in background.
- `alarm`: persistent reminder-like behavior plus looping alarm audio by default.
- `incomingCall`: special call-style UI with looping ringtone behavior.
- `urgent`: high-priority notification that can break through Focus Assist only where supported and allowed by user settings.

BenjaminTerm should avoid `alarm`, `incomingCall`, and `urgent` for normal agent-ready events. `reminder` is the only plausible sticky-toast experiment, but it needs a careful button/action design and should probably be an opt-in mode.

## Tag And Group

Windows uses `Tag` plus optional `Group` as a composite primary key for programmatic replacement and removal.

Implications:

- Same `Tag` plus same `Group` means the new notification replaces the old one.
- Replacement can reappear as a popup, but it is still semantically an update to the same notification identity.
- A fresh visible nag should use a fresh tag per agent-ready event.
- A pane should use a stable group so one user response can clear all outstanding ready reminders for that pane.

BenjaminTerm decision:

- Use a fresh tag for each agent-ready toast.
- Use a pane-scoped group such as `benjaminterm-agent-ready-pane-<pane_id>`.
- On user input to that pane, remove the whole group.

## Headers And Notification Center

Headers visually group related notifications in Notification Center, but do not change the app notification maximum or first-in-first-out behavior. Microsoft documents a 20-notification maximum per app and shows that headers do not alter that maximum.

Possible future use:

- A header per pane or window could make BenjaminTerm's Notification Center queue easier to scan.
- Headers are not necessary for the current release because pane groups already give us removal semantics.

## UX Guidance

Microsoft's UX guidance warns that notifications should be useful and not noisy. For BenjaminTerm, that means:

- Focused pane: sound plus visual pulse, no Windows toast.
- Background tab or other window: sound plus visual pulse plus toast.
- Repeated completion events: fresh toast so the user is visibly nagged again.
- User response: clear outstanding reminders for that pane.

This gives the desired Pavlovian workflow while respecting the user's intent once they return to the pane.
