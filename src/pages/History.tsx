import { useEffect, useState } from "react";
import { ipc, KillRecord } from "../lib/ipc";
import noKillYet from "../assets/nokillyet.png";

export default function History() {
  const [items, setItems] = useState<KillRecord[]>([]);

  const refresh = () => ipc.getHistory().then(setItems).catch(console.error);

  useEffect(() => {
    refresh();
    const id = setInterval(refresh, 2000);
    return () => clearInterval(id);
  }, []);

  const clear = async () => {
    await ipc.clearHistory();
    refresh();
  };

  return (
    <>
      <div className="page-header">
        <h2>Kill History</h2>
        <div className="subtitle">Recent termination attempts and their outcomes.</div>
      </div>
      <div className="card" style={{ padding: 0, overflow: "hidden" }}>
        {items.length === 0 ? (
          <div className="empty-state">
            <img src={noKillYet} alt="" className="empty-state-img" />
            <p>No kills yet. Hit your hotkey and click a window.</p>
          </div>
        ) : (
          <table>
            <thead>
              <tr>
                <th>Time</th>
                <th>Process</th>
                <th>PID</th>
                <th>Result</th>
              </tr>
            </thead>
            <tbody>
              {items.map((k, i) => (
                <tr key={i}>
                  <td style={{ color: "var(--fg-dim)" }}>{new Date(k.ts * 1000).toLocaleString()}</td>
                  <td style={{ fontWeight: 600 }}>{k.name}</td>
                  <td style={{ fontFamily: "ui-monospace, monospace", color: "var(--fg-dim)" }}>{k.pid}</td>
                  <td>
                    <span className={`badge ${k.success ? "killed" : "failed"}`}>
                      {k.success ? "killed" : k.reason || "failed"}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
      {items.length > 0 && (
        <div style={{ marginTop: 16 }}>
          <button className="ghost" onClick={clear}>Clear history</button>
        </div>
      )}
    </>
  );
}
