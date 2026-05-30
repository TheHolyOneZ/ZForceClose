import splash from "../assets/splash.png";
import appLogo from "../assets/killmode.png";
import { Github, Globe, ExternalLink } from "lucide-react";
import { ipc } from "../lib/ipc";

function Link({ href, icon: Icon, label, sub }: {
  href: string;
  icon: typeof Github;
  label: string;
  sub?: string;
}) {
  return (
    <button
      className="about-link"
      onClick={() => ipc.openUrl(href)}
      title={href}
    >
      <Icon size={18} strokeWidth={2} className="about-link-icon" />
      <div className="about-link-text">
        <span className="about-link-label">{label}</span>
        {sub && <span className="about-link-sub">{sub}</span>}
      </div>
      <ExternalLink size={13} className="about-link-arrow" />
    </button>
  );
}

export default function About() {
  return (
    <>
      <div className="page-header">
        <h2>About</h2>
        <div className="subtitle">Project info, links, and credits.</div>
      </div>

      <div className="about-hero" style={{ backgroundImage: `url(${splash})` }}>
        <div className="about-hero-overlay" />
        <img src={appLogo} alt="ZForceClose" className="about-logo-img" />
        <h1>ZForceClose</h1>
        <p className="tagline">"If an app won't close, point and click. No mercy."</p>
        <span className="version-pill">v0.1.0 · BETA</span>
      </div>

      <div className="about-grid">
        <div className="about-card">
          <div className="card-label">License</div>
          <div className="card-value">GPL-3.0-only</div>
          <div className="card-sub">Free software, free as in freedom.</div>
        </div>
        <div className="about-card">
          <div className="card-label">Copyright</div>
          <div className="card-value">© <span className="accent">TheHolyOneZ</span></div>
          <div className="card-sub">All rights reserved under GPL-3.0 terms.</div>
        </div>
        <div className="about-card">
          <div className="card-label">Platforms</div>
          <div className="card-value">Windows · Linux</div>
          <div className="card-sub">Win 10/11 and X11 sessions.</div>
        </div>
        <div className="about-card">
          <div className="card-label">Kill method</div>
          <div className="card-value">SIGKILL / TerminateProcess</div>
          <div className="card-sub">Instant termination. No grace period.</div>
        </div>
      </div>

      <div className="about-links-section">
        <div className="about-links-col">
          <div className="about-links-heading">Project</div>
          <Link
            href="https://zsync.eu/zforcekill/"
            icon={Globe}
            label="ZForceClose website"
            sub="zsync.eu/zforcekill/"
          />
          <Link
            href="https://github.com/TheHolyOneZ/ZForceClose"
            icon={Github}
            label="Source code"
            sub="github.com/TheHolyOneZ/ZForceClose"
          />
        </div>
        <div className="about-links-col">
          <div className="about-links-heading">Developer</div>
          <Link
            href="https://zsync.eu"
            icon={Globe}
            label="More tools & projects"
            sub="zsync.eu"
          />
          <Link
            href="https://github.com/TheHolyOneZ"
            icon={Github}
            label="TheHolyOneZ"
            sub="github.com/TheHolyOneZ"
          />
        </div>
      </div>

      <div className="about-credits">
        <div className="made-by">Developed & maintained by</div>
        <div className="dev-name">TheHolyOneZ</div>
        <div className="role">Creator · Lead Developer · Maintainer</div>
        <div className="tech-stack">
          <span className="tech-chip">Tauri v2</span>
          <span className="tech-chip">Rust</span>
          <span className="tech-chip">React 18</span>
          <span className="tech-chip">TypeScript</span>
          <span className="tech-chip">Vite</span>
        </div>
      </div>
    </>
  );
}
