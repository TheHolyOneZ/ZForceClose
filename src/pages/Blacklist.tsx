import { useState } from "react";
import { Config, ipc } from "../lib/ipc";

interface Props {
  config: Config;
  onChange: (next: Config) => void;
}

export default function Blacklist({ config, onChange }: Props) {
  const [text, setText] = useState(config.blacklist.join("\n"));

  const save = () => {
    const list = text.split("\n").map((s) => s.trim()).filter(Boolean);
    onChange({ ...config, blacklist: list });
  };

  const reset = async () => {
    const defaults = await ipc.defaultBlacklist();
    setText(defaults.join("\n"));
    onChange({ ...config, blacklist: defaults });
  };

  return (
    <>
      <div className="page-header">
        <h2>Blacklist</h2>
        <div className="subtitle">
          Process names (one per line) that ZForceClose will refuse to kill. Protects critical OS processes.
        </div>
      </div>
      <div className="card" style={{ padding: 20 }}>
        <textarea value={text} onChange={(e) => setText(e.target.value)} />
        <div style={{ display: "flex", gap: 10, marginTop: 16 }}>
          <button className="primary" onClick={save}>Save changes</button>
          <button className="ghost" onClick={reset}>Reset to defaults</button>
        </div>
      </div>
    </>
  );
}
