use crate::history::History;
use crate::{killer, window_pick};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager};

const ARM_TIMEOUT_MS: u64 = 8000;

pub struct KillModeState {
    armed: AtomicBool,
    armed_at_ms: AtomicU64,
    blacklist: Mutex<Vec<String>>,
}

impl KillModeState {
    pub fn new(blacklist: Vec<String>) -> Self {
        Self {
            armed: AtomicBool::new(false),
            armed_at_ms: AtomicU64::new(0),
            blacklist: Mutex::new(blacklist),
        }
    }

    pub fn set_blacklist(&self, list: Vec<String>) {
        *self.blacklist.lock() = list;
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}


pub fn trigger(app: AppHandle) {
    let state = app.state::<Arc<KillModeState>>().inner().clone();

    if state.armed.load(Ordering::SeqCst) {
        confirm_kill(&app, &state);
    } else {
        arm(&app, &state);
    }
}

fn arm(app: &AppHandle, state: &KillModeState) {
    state.armed.store(true, Ordering::SeqCst);
    state.armed_at_ms.store(now_ms(), Ordering::SeqCst);
    log::info!("Kill mode ARMED");
    let _ = app.emit("kill-mode-changed", true);

    if let Some(overlay) = app.get_webview_window("overlay") {
        let _ = overlay.show();
        let _ = overlay.set_always_on_top(true);
    }

    let app_clone = app.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ARM_TIMEOUT_MS));
        let state = app_clone.state::<Arc<KillModeState>>().inner().clone();
        let armed_at = state.armed_at_ms.load(Ordering::SeqCst);
        if state.armed.load(Ordering::SeqCst) && now_ms().saturating_sub(armed_at) >= ARM_TIMEOUT_MS {
            log::info!("Kill mode auto-timeout");
            cancel(&app_clone);
        }
    });
}

pub fn cancel(app: &AppHandle) {
    let state = app.state::<Arc<KillModeState>>().inner().clone();
    if state.armed.swap(false, Ordering::SeqCst) {
        log::info!("Kill mode DISARMED");
        let _ = app.emit("kill-mode-changed", false);
        if let Some(overlay) = app.get_webview_window("overlay") {
            let _ = overlay.hide();
        }
    }
}

fn confirm_kill(app: &AppHandle, state: &KillModeState) {
    state.armed.store(false, Ordering::SeqCst);
    let _ = app.emit("kill-mode-changed", false);
    log::info!("Confirm — killing window under cursor");

    let history = app.state::<Arc<History>>().inner().clone();
    let target = match window_pick::window_under_cursor() {
        Ok(t) => {
            log::info!("Target: {} (pid {})", t.name, t.pid);
            t
        }
        Err(e) => {
            log::warn!("window_under_cursor failed: {e}");
            let _ = app.emit("overlay-blocked", e.to_string());
            schedule_hide(app, 1600);
            return;
        }
    };

    if target.pid == std::process::id() {
        log::warn!("Refusing to kill self");
        let _ = app.emit("overlay-blocked", "Cannot kill ZForceClose itself".to_string());
        schedule_hide(app, 1600);
        return;
    }

    let bl = state.blacklist.lock().clone();
    if killer::is_blacklisted(&target.name, &bl) {
        log::warn!("Blacklisted: {}", target.name);
        history.push(target.pid, target.name.clone(), false, Some("blacklisted".into()));
        let _ = app.emit("overlay-blocked", format!("{} is blacklisted", target.name));
        schedule_hide(app, 1600);
        return;
    }

    match killer::kill(target.pid) {
        Ok(()) => {
            log::info!("KILLED {} ({})", target.name, target.pid);
            history.push(target.pid, target.name.clone(), true, None);
            let _ = app.emit("kill-success", target.name.clone());
            let _ = app.emit("overlay-killed", target.name);
            schedule_hide(app, 1400);
        }
        Err(e) => {
            log::error!("kill({}) failed: {e}", target.pid);
            history.push(target.pid, target.name.clone(), false, Some(e.to_string()));
            let _ = app.emit("overlay-blocked", e.to_string());
            schedule_hide(app, 1600);
        }
    }
}

fn schedule_hide(app: &AppHandle, ms: u64) {
    let app_clone = app.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms));
        if let Some(win) = app_clone.get_webview_window("overlay") {
            let _ = win.hide();
        }
    });
}
