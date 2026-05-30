interface Props {
  checked: boolean;
  onChange: (next: boolean) => void;
}

export default function Switch({ checked, onChange }: Props) {
  return (
    <label className="switch">
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      <span className="track">
        <span className="thumb" />
      </span>
    </label>
  );
}
