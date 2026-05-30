#[cfg(windows)]
mod windows_impl;
#[cfg(target_os = "linux")]
mod linux_impl;

#[derive(Debug, Clone)]
pub struct WindowTarget {
    pub pid: u32,
    pub name: String,
}

#[cfg(windows)]
pub fn window_under_cursor() -> anyhow::Result<WindowTarget> {
    windows_impl::window_under_cursor()
}

#[cfg(target_os = "linux")]
pub fn window_under_cursor() -> anyhow::Result<WindowTarget> {
    linux_impl::window_under_cursor()
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn window_under_cursor() -> anyhow::Result<WindowTarget> {
    anyhow::bail!("Unsupported platform")
}
