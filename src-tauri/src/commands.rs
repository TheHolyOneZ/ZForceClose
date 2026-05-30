use crate::config::{self, Config};
use crate::history::{History, KillRecord};
use crate::hotkey;
use crate::kill_mode::{self, KillModeState};
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, State};

pub struct ConfigState(pub Mutex<Config>);

#[tauri::command]
pub fn get_config(state: State<'_, ConfigState>) -> Config {
    state.0.lock().clone()
}

#[tauri::command]
pub fn set_config(
    app: AppHandle,
    cfg: Config,
    state: State<'_, ConfigState>,
    kill_state: State<'_, Arc<KillModeState>>,
    history: State<'_, Arc<History>>,
) -> Result<(), String> {
    let old = state.0.lock().clone();
    *state.0.lock() = cfg.clone();
    config::save(&app, &cfg).map_err(|e| e.to_string())?;

    kill_state.set_blacklist(cfg.blacklist.clone());
    history.set_max(cfg.history_max);

    if old.hotkey_enter_kill_mode != cfg.hotkey_enter_kill_mode {
        hotkey::register(&app, &cfg.hotkey_enter_kill_mode).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn trigger_kill(app: AppHandle) {
    kill_mode::trigger(app);
}

#[tauri::command]
pub fn cancel_kill_mode(app: AppHandle) {
    kill_mode::cancel(&app);
}

#[tauri::command]
pub fn get_history(history: State<'_, Arc<History>>) -> Vec<KillRecord> {
    history.snapshot()
}

#[tauri::command]
pub fn clear_history(history: State<'_, Arc<History>>) {
    history.clear();
}

#[tauri::command]
pub fn default_blacklist() -> Vec<String> {
    config::default_blacklist()
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
        use windows::core::HSTRING;
        unsafe {
            ShellExecuteW(
                None,
                &HSTRING::from("open"),
                &HSTRING::from(url.as_str()),
                None,
                None,
                SW_SHOWNORMAL,
            );
        }
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("xdg-open failed: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("open failed: {e}"))?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct SessionInfo {
    pub platform: String,
    pub session_type: String,
    pub supported: bool,
    pub warning: Option<String>,
}

#[tauri::command]
pub fn session_info() -> SessionInfo {
    #[cfg(target_os = "linux")]
    {
        let xdg = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let wayland = !std::env::var("WAYLAND_DISPLAY").unwrap_or_default().is_empty();
        let is_wayland = xdg.eq_ignore_ascii_case("wayland") || wayland;
        return SessionInfo {
            platform: "linux".into(),
            session_type: if is_wayland { "wayland".into() } else { "x11".into() },
            supported: !is_wayland,
            warning: if is_wayland {
                Some(
                    "Wayland is not supported. Wayland blocks cross-app window inspection for security, so ZForceClose cannot identify the window under your cursor. Log out and pick an X11 session at the login screen.".into(),
                )
            } else {
                None
            },
        };
    }
    #[cfg(target_os = "windows")]
    {
        SessionInfo {
            platform: "windows".into(),
            session_type: "win32".into(),
            supported: true,
            warning: None,
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        SessionInfo {
            platform: "other".into(),
            session_type: "unknown".into(),
            supported: false,
            warning: Some("This OS is not supported.".into()),
        }
    }
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let mgr = app.autolaunch();
    if enabled {
        mgr.enable().map_err(|e| e.to_string())
    } else {
        mgr.disable().map_err(|e| e.to_string())
    }
}
