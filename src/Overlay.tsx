import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import killmode from "./assets/killmode.png";

type View =
  | { kind: "armed"; armedAt: number }
  | { kind: "killed"; text: string }
  | { kind: "blocked"; text: string }
  | null;

const TIMEOUT_MS = 8000;

export default function Overlay() {
  const [view, setView] = useState<View>(null);
  const [remaining, setRemaining] = useState(TIMEOUT_MS);

  useEffect(() => {
    document.documentElement.style.background = "transparent";
    document.body.style.background = "transparent";
    document.body.style.overflow = "hidden";
    document.body.style.margin = "0";

    const u1 = listen<boolean>("kill-mode-changed", (e) => {
      if (e.payload) setView({ kind: "armed", armedAt: Date.now() });
      else setView((v) => (v?.kind === "armed" ? null : v));
    });
    const u2 = listen<string>("overlay-killed", (e) =>
      setView({ kind: "killed", text: e.payload })
    );
    const u3 = listen<string>("overlay-blocked", (e) =>
      setView({ kind: "blocked", text: e.payload })
    );
    return () => {
      u1.then((f) => f());
      u2.then((f) => f());
      u3.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (view?.kind !== "armed") return;
    const tick = () => setRemaining(Math.max(0, TIMEOUT_MS - (Date.now() - view.armedAt)));
    tick();
    const id = setInterval(tick, 100);
    return () => clearInterval(id);
  }, [view]);

  if (!view) return null;

  if (view.kind === "armed") {
    const pct = (remaining / TIMEOUT_MS) * 100;
    return (
      <div className="overlay-widget overlay-armed" onClick={() => invoke("cancel_kill_mode")}>
        <img src={killmode} className="overlay-skull" alt="" />
        <div className="overlay-info">
          <div className="overlay-title">
            <span className="overlay-dot" />KILL MODE ARMED
          </div>
          <div className="overlay-status">Aim cursor at target, press hotkey again</div>
          <div className="overlay-hint">Click here or wait {Math.ceil(remaining / 1000)}s to cancel</div>
          <div className="overlay-progress"><div className="overlay-progress-bar" style={{ width: `${pct}%` }} /></div>
        </div>
      </div>
    );
  }

  return (
    <div className={`overlay-widget overlay-${view.kind}`}>
      <img src={killmode} className="overlay-skull" alt="" />
      <div className="overlay-info">
        <div className="overlay-title">
          <span className="overlay-dot" />
          {view.kind === "killed" ? "TARGET ELIMINATED" : "BLOCKED"}
        </div>
        <div className="overlay-status">{view.text}</div>
      </div>
    </div>
  );
}
