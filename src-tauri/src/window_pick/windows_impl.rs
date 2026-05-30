use super::WindowTarget;
use anyhow::{anyhow, Result};
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, MAX_PATH, POINT};
use windows::Win32::System::ProcessStatus::GetModuleBaseNameW;
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetAncestor, GetCursorPos, GetWindowThreadProcessId, WindowFromPoint, GA_ROOT,
};

pub fn window_under_cursor() -> Result<WindowTarget> {
    unsafe {
        let mut pt = POINT::default();
        GetCursorPos(&mut pt).map_err(|e| anyhow!("GetCursorPos: {e}"))?;

        let hwnd: HWND = WindowFromPoint(pt);
        if hwnd.0.is_null() {
            return Err(anyhow!("No window under cursor"));
        }
        let top = GetAncestor(hwnd, GA_ROOT);
        let target = if top.0.is_null() { hwnd } else { top };

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(target, Some(&mut pid));
        if pid == 0 {
            return Err(anyhow!("Could not resolve PID"));
        }

        let name = process_name(pid).unwrap_or_else(|| format!("pid:{pid}"));

        if name.eq_ignore_ascii_case("ApplicationFrameHost.exe") {
            let real = uwp_real_pid(target).unwrap_or(pid);
            let real_name = process_name(real).unwrap_or_else(|| format!("pid:{real}"));
            return Ok(WindowTarget { pid: real, name: real_name });
        }

        can_we_kill(pid)?;

        Ok(WindowTarget { pid, name })
    }
}

unsafe fn can_we_kill(pid: u32) -> Result<()> {
    use windows::Win32::System::Threading::PROCESS_TERMINATE;
    let h = OpenProcess(PROCESS_TERMINATE, false, pid);
    match h {
        Ok(handle) => {
            let _ = CloseHandle(handle);
            Ok(())
        }
        Err(e) => {
            let code = e.code().0 as u32;
            if code == 0x80070005 {
                Err(anyhow!(
                    "Access denied — target process is running as Administrator. \
                     Restart ZForceClose as Administrator to kill elevated processes."
                ))
            } else {
                Err(anyhow!("Cannot open process for termination: {e}"))
            }
        }
    }
}

unsafe fn uwp_real_pid(frame_hwnd: HWND) -> Option<u32> {
    use windows::Win32::UI::WindowsAndMessaging::{EnumChildWindows, GetWindowThreadProcessId};

    struct State { pid: u32 }
    unsafe extern "system" fn enum_cb(hwnd: HWND, lparam: LPARAM) -> windows::Win32::Foundation::BOOL {
        let state = &mut *(lparam.0 as *mut State);
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid != 0 {
            let name = process_name_raw(pid).unwrap_or_default();
            if !name.eq_ignore_ascii_case("ApplicationFrameHost.exe") {
                state.pid = pid;
                return false.into();
            }
        }
        true.into()
    }

    let mut state = State { pid: 0 };
    let _ = EnumChildWindows(frame_hwnd, Some(enum_cb), LPARAM(&mut state as *mut _ as isize));
    if state.pid != 0 { Some(state.pid) } else { None }
}

fn process_name(pid: u32) -> Option<String> {
    unsafe { process_name_raw(pid) }
}

unsafe fn process_name_raw(pid: u32) -> Option<String> {
    let handle = OpenProcess(
        PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
        false,
        pid,
    )
    .ok()?;
    let mut buf = [0u16; MAX_PATH as usize];
    let len = GetModuleBaseNameW(handle, None, &mut buf);
    let _ = CloseHandle(handle);
    if len == 0 {
        return None;
    }
    Some(String::from_utf16_lossy(&buf[..len as usize]))
}
