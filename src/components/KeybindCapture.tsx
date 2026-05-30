import { useState } from "react";

interface Props {
  value: string;
  onChange: (next: string) => void;
}

export default function KeybindCapture({ value, onChange }: Props) {
  const [recording, setRecording] = useState(false);

  const handleKey = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (!recording) return;
    e.preventDefault();
    const parts: string[] = [];
    if (e.ctrlKey) parts.push("Ctrl");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");
    if (e.metaKey) parts.push("Super");
    const key = e.key;
    if (!["Control", "Shift", "Alt", "Meta"].includes(key)) {
      parts.push(key.length === 1 ? key.toUpperCase() : key);
      onChange(parts.join("+"));
      setRecording(false);
    }
  };

  return (
    <button
      className="ghost"
      onKeyDown={handleKey}
      onClick={() => setRecording(true)}
      onBlur={() => setRecording(false)}
    >
      {recording ? "Press keys…" : <span className="kbd">{value}</span>}
    </button>
  );
}
