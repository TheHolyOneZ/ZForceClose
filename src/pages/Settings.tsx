import Switch from "../components/Switch";
import { Config, ipc } from "../lib/ipc";
import { FlaskConical } from "lucide-react";

interface Props {
  config: Config;
  onChange: (next: Config) => void;
}

function ExperimentalBadge() {
  return (
    <span className="badge-experimental">
      <FlaskConical size={11} strokeWidth={2.5} />
      Experimental
    </span>
  );
}

export default function Settings({ config, onChange }: Props) {
  const toggle = (key: keyof Config) => async (val: boolean) => {
    const next = { ...config, [key]: val } as Config;
    onChange(next);
    if (key === "start_at_login") await ipc.setAutostart(val);
  };

  return (
    <>
      <div className="page-header">
        <h2>Settings</h2>
        <div className="subtitle">General preferences and feedback options.</div>
      </div>

      <div className="card">
        <div className="row">
          <div className="row-info">
            <div className="label">Start at login</div>
            <div className="desc">Launch ZForceClose when you sign in.</div>
          </div>
          <div className="row-action">
            <Switch checked={config.start_at_login} onChange={toggle("start_at_login")} />
          </div>
        </div>
        <div className="row">
          <div className="row-info">
            <div className="label">Start minimized to tray</div>
            <div className="desc">Don't open the main window on startup.</div>
          </div>
          <div className="row-action">
            <Switch checked={config.start_minimized_to_tray} onChange={toggle("start_minimized_to_tray")} />
          </div>
        </div>
        <div className="row">
          <div className="row-info">
            <div className="label">History size</div>
            <div className="desc">Maximum number of recent kills to remember.</div>
          </div>
          <div className="row-action">
            <input
              type="number"
              min={10}
              max={1000}
              value={config.history_max}
              style={{ width: 100, textAlign: "center" }}
              onChange={(e) => onChange({ ...config, history_max: parseInt(e.target.value || "100", 10) })}
            />
          </div>
        </div>
      </div>

      <div className="section-label">
        <FlaskConical size={13} strokeWidth={2.5} />
        Experimental Features
      </div>
      <div className="card experimental-card">
        <div className="experimental-notice">
          These features are unfinished and may not work on all systems. Enable at your own risk.
        </div>
        <div className="row">
          <div className="row-info">
            <div className="label-row">
              <span className="label">Play sound on kill</span>
              <ExperimentalBadge />
            </div>
            <div className="desc">Audio feedback when a process is terminated. May not work on all setups.</div>
          </div>
          <div className="row-action">
            <Switch checked={config.play_sound_on_kill} onChange={toggle("play_sound_on_kill")} />
          </div>
        </div>
        <div className="row">
          <div className="row-info">
            <div className="label-row">
              <span className="label">Flash on kill</span>
              <ExperimentalBadge />
            </div>
            <div className="desc">Brief screen flash for visual confirmation. May interfere with some compositors.</div>
          </div>
          <div className="row-action">
            <Switch checked={config.flash_on_kill} onChange={toggle("flash_on_kill")} />
          </div>
        </div>
      </div>
    </>
  );
}
