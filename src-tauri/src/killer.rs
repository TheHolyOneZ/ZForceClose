use anyhow::{anyhow, Result};

pub fn kill(pid: u32) -> Result<()> {
    #[cfg(windows)]
    {
        kill_windows(pid)
    }
    #[cfg(target_os = "linux")]
    {
        kill_linux(pid)
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        let _ = pid;
        Err(anyhow!("Unsupported platform"))
    }
}

#[cfg(windows)]
fn kill_windows(pid: u32) -> Result<()> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, false, pid)
            .map_err(|e| anyhow!("OpenProcess({pid}): {e}"))?;
        let res = TerminateProcess(handle, 1);
        let _ = CloseHandle(handle);
        res.map_err(|e| anyhow!("TerminateProcess({pid}): {e}"))?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
fn kill_linux(pid: u32) -> Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    kill(Pid::from_raw(pid as i32), Signal::SIGKILL)
        .map_err(|e| anyhow!("SIGKILL({pid}): {e}"))?;
    Ok(())
}

pub fn is_blacklisted(name: &str, blacklist: &[String]) -> bool {
    let lower = name.to_lowercase();
    let lower = lower.trim_end_matches(".exe");
    blacklist.iter().any(|b| {
        let bl = b.to_lowercase();
        let bl = bl.trim_end_matches(".exe");
        bl == lower
    })
}
