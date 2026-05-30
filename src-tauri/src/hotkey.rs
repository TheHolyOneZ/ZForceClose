use crate::kill_mode;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn register(app: &AppHandle, accelerator: &str) -> anyhow::Result<()> {
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    let shortcut: Shortcut = accelerator
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid shortcut '{accelerator}': {e:?}"))?;

    log::info!("Registering global shortcut: {accelerator} -> {shortcut:?}");

    let app_clone = app.clone();
    gs.on_shortcut(shortcut, move |_app, _shortcut, event| {
        if event.state() == ShortcutState::Pressed {
            kill_mode::trigger(app_clone.clone());
        }
    })
    .map_err(|e| anyhow::anyhow!("register shortcut '{accelerator}': {e}"))?;

    Ok(())
}
