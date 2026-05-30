use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hotkey_enter_kill_mode: String,
    pub start_at_login: bool,
    pub start_minimized_to_tray: bool,
    pub play_sound_on_kill: bool,
    pub flash_on_kill: bool,
    pub blacklist: Vec<String>,
    pub history_max: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hotkey_enter_kill_mode: "Ctrl+Shift+K".to_string(),
            start_at_login: false,
            start_minimized_to_tray: true,
            play_sound_on_kill: true,
            flash_on_kill: true,
            blacklist: default_blacklist(),
            history_max: 100,
        }
    }
}

pub fn default_blacklist() -> Vec<String> {
    [

        "explorer.exe",
        "winlogon.exe",
        "csrss.exe",
        "wininit.exe",
        "services.exe",
        "lsass.exe",
        "smss.exe",
        "dwm.exe",

        "systemd",
        "Xorg",
        "Xwayland",
        "gnome-shell",
        "kwin_x11",
        "kwin_wayland",
        "plasmashell",

        "ZForceClose",
        "zforceclose",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn config_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_config_dir()
        .expect("config dir");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir.join("settings.json")
}

pub fn load(app: &AppHandle) -> Config {
    let path = config_path(app);
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<Config>(&data) {
            return cfg;
        }
    }
    let cfg = Config::default();
    let _ = save(app, &cfg);
    cfg
}

pub fn save(app: &AppHandle, cfg: &Config) -> anyhow::Result<()> {
    let path = config_path(app);
    let data = serde_json::to_string_pretty(cfg)?;
    fs::write(path, data)?;
    Ok(())
}
