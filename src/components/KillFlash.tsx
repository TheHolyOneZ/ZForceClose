import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import killConfirmedSrc from "../assets/killconfirmed.png";

export default function KillFlash() {
  const [flash, setFlash] = useState<{ name: string; key: number } | null>(null);

  useEffect(() => {
    const unlisten = listen<string>("kill-success", (e) => {
      setFlash({ name: e.payload, key: Date.now() });
      setTimeout(() => setFlash(null), 1100);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (!flash) return null;

  return (
    <div className="kill-flash" key={flash.key}>
      <img src={killConfirmedSrc} alt="" className="kill-flash-img" />
      <div className="kill-flash-text">
        <div className="kill-flash-label">KILLED</div>
        <div className="kill-flash-name">{flash.name}</div>
      </div>
    </div>
  );
}
