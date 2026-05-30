import KeybindCapture from "../components/KeybindCapture";
import { Config } from "../lib/ipc";

interface Props {
  config: Config;
  onChange: (next: Config) => void;
}

export default function Hotkeys({ config, onChange }: Props) {
  return (
    <>
      <div className="page-header">
        <h2>Hotkeys</h2>
        <div className="subtitle">
          Press to arm. Aim cursor at the target window. Press again to kill.
        </div>
      </div>
      <div className="card">
        <div className="row">
          <div className="row-info">
            <div className="label">Arm / Confirm kill</div>
            <div className="desc">
              First press arms kill mode. Second press kills the window under your cursor. Must include a modifier (Ctrl/Shift/Alt) — single keys won't register globally.
            </div>
          </div>
          <div className="row-action">
            <KeybindCapture
              value={config.hotkey_enter_kill_mode}
              onChange={(v) => onChange({ ...config, hotkey_enter_kill_mode: v })}
            />
          </div>
        </div>
        <div className="row">
          <div className="row-info">
            <div className="label">Cancel</div>
            <div className="desc">Click the floating overlay widget, or wait 8 seconds for auto-cancel.</div>
          </div>
          <div className="row-action"><span className="kbd">click overlay</span></div>
        </div>
      </div>
    </>
  );
}
