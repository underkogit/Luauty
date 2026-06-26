use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use mlua::prelude::*;
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, WPARAM};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess};
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyWindow, EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    IsWindow, IsWindowVisible, PostMessageW, SendMessageW, WM_CLOSE,
};
use windows::core::BOOL;

fn wide_to_string(wide: &[u16]) -> String {
    OsString::from_wide(wide).to_string_lossy().into_owned()
}

fn hwnd_from_usize(hwnd: usize) -> HWND {
    HWND(hwnd as *mut core::ffi::c_void)
}

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let find_window_by_text = lua.create_function(|_, query: String| {
        struct Ctx {
            query: String,
            found: usize,
        }

        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let ctx = &mut *(lparam.0 as *mut Ctx);

            if !IsWindowVisible(hwnd).as_bool() {
                return BOOL(1);
            }

            let len = GetWindowTextLengthW(hwnd);
            if len <= 0 {
                return BOOL(1);
            }

            let mut buf = vec![0u16; (len + 1) as usize];
            let written = GetWindowTextW(hwnd, &mut buf);
            if written <= 0 {
                return BOOL(1);
            }

            let title = wide_to_string(&buf[..written as usize]);
            if title.contains(&ctx.query) {
                ctx.found = hwnd.0 as usize;
                return BOOL(0);
            }

            BOOL(1)
        }

        let mut ctx = Ctx { query, found: 0 };

        unsafe {
            EnumWindows(Some(enum_proc), LPARAM(&mut ctx as *mut Ctx as isize));
        }

        Ok(ctx.found)
    })?;

    let get_window_process_id = lua.create_function(|_, hwnd: usize| {
        let hwnd = hwnd_from_usize(hwnd);

        let mut pid = 0u32;
        unsafe {
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
        }

        Ok(pid)
    })?;

    let find_windows = lua.create_function(|lua, query: String| {
        struct Ctx {
            query: String,
            found: Vec<usize>,
        }

        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let ctx = &mut *(lparam.0 as *mut Ctx);

            if !IsWindowVisible(hwnd).as_bool() {
                return BOOL(1);
            }

            let len = GetWindowTextLengthW(hwnd);
            if len <= 0 {
                return BOOL(1);
            }

            let mut buf = vec![0u16; (len + 1) as usize];
            let written = GetWindowTextW(hwnd, &mut buf);
            if written <= 0 {
                return BOOL(1);
            }

            let title = wide_to_string(&buf[..written as usize]);
            if title.contains(&ctx.query) {
                ctx.found.push(hwnd.0 as usize);
            }

            BOOL(1)
        }

        let mut ctx = Ctx {
            query,
            found: Vec::new(),
        };

        unsafe {
            EnumWindows(Some(enum_proc), LPARAM(&mut ctx as *mut Ctx as isize));
        }

        let arr = lua.create_table()?;
        for (i, hwnd) in ctx.found.into_iter().enumerate() {
            arr.set(i + 1, hwnd)?;
        }

        Ok(arr)
    })?;

    let get_window_title = lua.create_function(|_, hwnd: usize| {
        let hwnd = HWND(hwnd as *mut core::ffi::c_void);

        let len = unsafe { GetWindowTextLengthW(hwnd) };
        if len <= 0 {
            return Ok(String::new());
        }

        let mut buf = vec![0u16; (len + 1) as usize];
        let written = unsafe { GetWindowTextW(hwnd, &mut buf) };
        if written <= 0 {
            return Ok(String::new());
        }

        Ok(wide_to_string(&buf[..written as usize]))
    })?;

    let close_window = lua.create_function(|_, hwnd: usize| {
        let hwnd = HWND(hwnd as *mut core::ffi::c_void);

        unsafe {
            SendMessageW(hwnd, WM_CLOSE, Some(WPARAM(0)), Some(LPARAM(0)));
            if IsWindow(Some(hwnd)).as_bool() {
                PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
            }
            if IsWindow(Some(hwnd)).as_bool() {
                DestroyWindow(hwnd);
            }
        }

        Ok(())
    })?;

    let close_process = lua.create_function(|_, pid: u32| {
        let handle = unsafe { OpenProcess(PROCESS_TERMINATE, false, pid) }
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        unsafe {
            TerminateProcess(handle, 0).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            CloseHandle(handle);
        }

        Ok(())
    })?;

    globals.set("find_window_by_text", find_window_by_text)?;
    globals.set("get_window_process_id", get_window_process_id)?;
    globals.set("find_windows", find_windows)?;
    globals.set("get_window_title", get_window_title)?;
    globals.set("close_window", close_window)?;
    globals.set("close_process", close_process)?;

    Ok(())
}
