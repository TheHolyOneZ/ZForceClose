mod commands;
mod config;
mod history;
mod hotkey;
mod kill_mode;
mod killer;
mod tray;
mod window_pick;

use commands::ConfigState;
use history::History;
use kill_mode::KillModeState;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .try_init();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::set_config,
            commands::trigger_kill,
            commands::cancel_kill_mode,
            commands::session_info,
            commands::open_url,
            commands::get_history,
            commands::clear_history,
            commands::default_blacklist,
            commands::set_autostart,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            let cfg = config::load(&handle);

            let history = Arc::new(History::new(cfg.history_max));
            let kill_state = Arc::new(KillModeState::new(cfg.blacklist.clone()));

            app.manage(ConfigState(Mutex::new(cfg.clone())));
            app.manage(history);
            app.manage(kill_state);

            if let Err(e) = tray::build(&handle) {
                log::error!("tray build failed: {e}");
            }
            if let Err(e) = hotkey::register(&handle, &cfg.hotkey_enter_kill_mode) {
                log::error!("hotkey register failed: {e}");
            }

            if cfg.start_minimized_to_tray {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.hide();
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
