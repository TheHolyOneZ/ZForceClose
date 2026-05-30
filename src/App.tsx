import { useEffect, useState } from "react";
import { Keyboard, Settings as SettingsIcon, Shield, History as HistoryIcon, Info, Skull, AlertTriangle } from "lucide-react";
import Hotkeys from "./pages/Hotkeys";
import Settings from "./pages/Settings";
import Blacklist from "./pages/Blacklist";
import History from "./pages/History";
import About from "./pages/About";
import KillFlash from "./components/KillFlash";
import { ipc, Config, SessionInfo } from "./lib/ipc";

type Tab = "hotkeys" | "settings" | "blacklist" | "history" | "about";

const TABS: { id: Tab; label: string; Icon: typeof Keyboard }[] = [
  { id: "hotkeys", label: "Hotkeys", Icon: Keyboard },
  { id: "settings", label: "Settings", Icon: SettingsIcon },
  { id: "blacklist", label: "Blacklist", Icon: Shield },
  { id: "history", label: "History", Icon: HistoryIcon },
  { id: "about", label: "About", Icon: Info },
];

export default function App() {
  const [tab, setTab] = useState<Tab>("hotkeys");
  const [config, setConfig] = useState<Config | null>(null);
  const [session, setSession] = useState<SessionInfo | null>(null);

  useEffect(() => {
    ipc.getConfig().then(setConfig).catch(console.error);
    ipc.sessionInfo().then(setSession).catch(console.error);
  }, []);

  const updateConfig = async (next: Config) => {
    setConfig(next);
    await ipc.setConfig(next);
  };

  return (
    <div className="app">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-logo">
            <Skull size={18} strokeWidth={2.5} />
          </div>
          <div>
            <div className="brand-name">ZForceClose</div>
            <div className="brand-version">v0.1.0</div>
          </div>
        </div>
        <nav className="nav">
          {TABS.map((t) => (
            <button
              key={t.id}
              className={tab === t.id ? "active" : ""}
              onClick={() => setTab(t.id)}
            >
              <t.Icon size={16} strokeWidth={2} className="nav-icon" />
              {t.label}
            </button>
          ))}
        </nav>
        <div className="sidebar-footer">
          <div><span className="status-dot" />Listening for hotkey</div>
        </div>
      </aside>
      <main className="content">
        {session?.warning && (
          <div className="session-warning">
            <AlertTriangle size={18} strokeWidth={2.5} />
            <div>
              <div className="session-warning-title">{session.session_type.toUpperCase()} session detected</div>
              <div className="session-warning-body">{session.warning}</div>
            </div>
          </div>
        )}
        {!config ? (
          <p>Loading…</p>
        ) : (
          <>
            {tab === "hotkeys" && <Hotkeys config={config} onChange={updateConfig} />}
            {tab === "settings" && <Settings config={config} onChange={updateConfig} />}
            {tab === "blacklist" && <Blacklist config={config} onChange={updateConfig} />}
            {tab === "history" && <History />}
            {tab === "about" && <About />}
          </>
        )}
      </main>
      <KillFlash />
    </div>
  );
}
