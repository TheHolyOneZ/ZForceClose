# ZForceClose

> No-mercy window killer for Windows 10/11 and Linux (X11).

[![Download](https://img.shields.io/badge/Download-zsync.eu%2Fzforcekill-ef4444?style=flat-square)](https://zsync.eu/zforcekill/)
[![GitHub](https://img.shields.io/badge/GitHub-TheHolyOneZ%2FZForceClose-24292e?style=flat-square&logo=github)](https://github.com/TheHolyOneZ/ZForceClose)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue?style=flat-square)](LICENSE)

When an app refuses to die, press your hotkey to arm ZForceClose, move your cursor over the frozen window, and press again. The process is terminated instantly — `TerminateProcess` on Windows, `SIGKILL` on Linux. No dialogs. No grace period. No mercy.

## Download

**→ [zsync.eu/zforcekill/](https://zsync.eu/zforcekill/)**

| Platform | Format | Link |
|---|---|---|
| Windows 10/11 | Installer (.exe) | [zsync.eu/zforcekill/release/](https://zsync.eu/zforcekill/release/) |
| Windows 10/11 | MSI Package | [zsync.eu/zforcekill/release/](https://zsync.eu/zforcekill/release/) |
| Linux (X11) | AppImage | [zsync.eu/zforcekill/release/](https://zsync.eu/zforcekill/release/) |
| Linux (X11) | .deb (Debian/Ubuntu) | [zsync.eu/zforcekill/release/](https://zsync.eu/zforcekill/release/) |
| Linux (X11) | .rpm (Fedora/RHEL) | [zsync.eu/zforcekill/release/](https://zsync.eu/zforcekill/release/) |

## How it works

1. Launch ZForceClose — it lives in your system tray.
2. Press your global hotkey (default `Ctrl+Shift+K`) to **arm** kill mode.
3. Move your cursor over the frozen window.
4. Press the hotkey again — the process is terminated instantly.
5. Click the overlay widget or wait 8 seconds to cancel.

## Features

- Configurable global hotkey (arm + confirm, two-press flow)
- System tray with quick-kill action
- Process blacklist (protects `explorer.exe`, `systemd`, your DE compositor, etc.)
- Kill history log with timestamps and results
- Start at login (Windows registry / Linux XDG autostart)
- Experimental: sound + screen flash on kill

## Build from source

Requires Rust, Node 18+, and the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your platform.

```bash
pnpm install
pnpm tauri dev      # development
pnpm tauri build    # release bundle
```

## Platform notes

- **Wayland is not supported in v1.** ZForceClose needs to resolve the window under the cursor — Wayland blocks cross-app window inspection by design. Switch to an X11 session on Linux.
- On Linux, you may need `libx11-dev` / `libxcb*-dev` packages.
- Killing elevated (admin) processes on Windows requires running ZForceClose as Administrator.

## Links

| | |
|---|---|
| Website | [zsync.eu/zforcekill/](https://zsync.eu/zforcekill/) |
| More tools by the same dev | [zsync.eu](https://zsync.eu) |
| GitHub | [github.com/TheHolyOneZ/ZForceClose](https://github.com/TheHolyOneZ/ZForceClose) |
| Developer | [github.com/TheHolyOneZ](https://github.com/TheHolyOneZ) |

## License

GPL-3.0-only — © TheHolyOneZ. See [LICENSE](LICENSE).
