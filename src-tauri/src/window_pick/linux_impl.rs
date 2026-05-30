use super::WindowTarget;
use anyhow::{anyhow, Result};
use std::fs;
use x11rb::connection::Connection;
use x11rb::protocol::res::{ClientIdMask, ClientIdSpec, ConnectionExt as ResConnExt};
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, MapState, Window};

pub fn window_under_cursor() -> Result<WindowTarget> {
    if is_wayland() {
        return Err(anyhow!(
            "Wayland sessions are not supported. Switch to an X11 session."
        ));
    }

    let (conn, screen_num) =
        x11rb::connect(None).map_err(|e| anyhow!("X11 connect failed: {e}"))?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let pointer = conn
        .query_pointer(root)?
        .reply()
        .map_err(|e| anyhow!("query_pointer: {e}"))?;

    let px = pointer.root_x as i32;
    let py = pointer.root_y as i32;

    let direct = if pointer.child != 0 {
        pointer.child
    } else {
        find_window_at(&conn, root, px, py).ok_or_else(|| {
            anyhow!("Cursor is over the desktop — no window to kill")
        })?
    };
    let toplevel = top_level(&conn, root, direct).unwrap_or(direct);

    let pid_atom = conn.intern_atom(false, b"_NET_WM_PID")?.reply()?.atom;

    let strategies: [(&str, Window); 4] = [
        ("_NET_WM_PID on toplevel", toplevel),
        ("XRes on toplevel", toplevel),
        ("_NET_WM_PID on direct", direct),
        ("XRes on direct", direct),
    ];

    let mut pid: Option<u32> = None;
    let mut found_via = "";
    let mut name_win = toplevel;

    for (i, &(label, w)) in strategies.iter().enumerate() {
        let candidate = if i % 2 == 0 {
            read_net_wm_pid(&conn, w, pid_atom)
        } else {
            xres_pid(&conn, w)
        };
        if let Some(p) = candidate {
            if owned_by_local_user(p) {
                pid = Some(p);
                found_via = label;
                name_win = w;
                break;
            }
        }
    }

    let pid = pid.ok_or_else(|| {
        anyhow!("Could not resolve PID for the window under cursor — try clicking the window first to focus it, then retry")
    })?;

    log::info!("Picker resolved PID {pid} via {found_via}");

    let name = process_name(pid)
        .or_else(|| wm_class(&conn, name_win))
        .unwrap_or_else(|| format!("pid:{pid}"));

    Ok(WindowTarget { pid, name })
}

fn read_net_wm_pid(conn: &impl Connection, win: Window, pid_atom: u32) -> Option<u32> {
    let prop = conn
        .get_property(false, win, pid_atom, AtomEnum::CARDINAL, 0, 1)
        .ok()?
        .reply()
        .ok()?;
    if prop.value_len == 0 || prop.format != 32 {
        return None;
    }
    let vals: Vec<u32> = prop.value32()?.collect();
    vals.into_iter().next().filter(|&p| p > 0)
}

fn xres_pid(conn: &impl Connection, win: Window) -> Option<u32> {
    let specs = vec![ClientIdSpec {
        client: win,
        mask: ClientIdMask::LOCAL_CLIENT_PID.into(),
    }];
    let reply = conn.res_query_client_ids(&specs).ok()?.reply().ok()?;
    let target_mask: u32 = ClientIdMask::LOCAL_CLIENT_PID.into();
    for id in reply.ids {
        let id_mask: u32 = id.spec.mask.into();
        if id_mask & target_mask != 0 {
            if let Some(&pid) = id.value.first() {
                if pid > 0 {
                    return Some(pid);
                }
            }
        }
    }
    None
}

fn find_window_at(conn: &impl Connection, root: Window, px: i32, py: i32) -> Option<Window> {
    let tree = conn.query_tree(root).ok()?.reply().ok()?;
    for &win in tree.children.iter().rev() {
        let attrs = match conn.get_window_attributes(win).ok().and_then(|c| c.reply().ok()) {
            Some(a) => a,
            None => continue,
        };
        if attrs.map_state != MapState::VIEWABLE {
            continue;
        }
        let geom = match conn.get_geometry(win).ok().and_then(|c| c.reply().ok()) {
            Some(g) => g,
            None => continue,
        };
        let coords = match conn
            .translate_coordinates(win, root, 0, 0)
            .ok()
            .and_then(|c| c.reply().ok())
        {
            Some(c) => c,
            None => continue,
        };
        let x = coords.dst_x as i32;
        let y = coords.dst_y as i32;
        let w = geom.width as i32;
        let h = geom.height as i32;
        if px >= x && px < x + w && py >= y && py < y + h {
            log::info!("find_window_at hit window 0x{win:x} at ({x},{y}) {w}x{h}");
            return Some(win);
        }
    }
    None
}

fn top_level(conn: &impl Connection, root: Window, mut win: Window) -> Result<Window> {
    for _ in 0..32 {
        let tree = conn.query_tree(win)?.reply()?;
        if tree.parent == root || tree.parent == 0 {
            return Ok(win);
        }
        win = tree.parent;
    }
    Ok(win)
}

fn owned_by_local_user(pid: u32) -> bool {
    fs::metadata(format!("/proc/{pid}")).is_ok()
}

fn wm_class(conn: &impl Connection, win: Window) -> Option<String> {
    let prop = conn
        .get_property(false, win, AtomEnum::WM_CLASS, AtomEnum::STRING, 0, 256)
        .ok()?
        .reply()
        .ok()?;
    if prop.value_len == 0 {
        return None;
    }
    let s = String::from_utf8_lossy(&prop.value).to_string();
    s.split('\0').find(|p| !p.is_empty()).map(|p| p.to_string())
}

fn process_name(pid: u32) -> Option<String> {
    fs::read_to_string(format!("/proc/{pid}/comm"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn is_wayland() -> bool {
    if std::env::var("XDG_SESSION_TYPE")
        .map(|v| v.eq_ignore_ascii_case("wayland"))
        .unwrap_or(false)
    {
        return true;
    }
    std::env::var("WAYLAND_DISPLAY").is_ok()
}
