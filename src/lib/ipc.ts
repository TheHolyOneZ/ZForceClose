import { invoke } from "@tauri-apps/api/core";

export interface Config {
  hotkey_enter_kill_mode: string;
  start_at_login: boolean;
  start_minimized_to_tray: boolean;
  play_sound_on_kill: boolean;
  flash_on_kill: boolean;
  blacklist: string[];
  history_max: number;
}

export interface KillRecord {
  pid: number;
  name: string;
  ts: number;
  success: boolean;
  reason?: string;
}

export interface SessionInfo {
  platform: string;
  session_type: string;
  supported: boolean;
  warning: string | null;
}

export const ipc = {
  getConfig: () => invoke<Config>("get_config"),
  setConfig: (cfg: Config) => invoke<void>("set_config", { cfg }),
  triggerKill: () => invoke<void>("trigger_kill"),
  cancelKillMode: () => invoke<void>("cancel_kill_mode"),
  getHistory: () => invoke<KillRecord[]>("get_history"),
  clearHistory: () => invoke<void>("clear_history"),
  setAutostart: (enabled: boolean) => invoke<void>("set_autostart", { enabled }),
  defaultBlacklist: () => invoke<string[]>("default_blacklist"),
  sessionInfo: () => invoke<SessionInfo>("session_info"),
  openUrl: (url: string) => invoke<void>("open_url", { url }),
};
