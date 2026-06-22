use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use mlua::prelude::*;
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindow, IsWindowVisible, PostMessageW, SendMessageW, WM_CLOSE};

fn wide_to_string(wide: &[u16]) -> String {
    OsString::from_wide(wide).to_string_lossy().into_owned()
}

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let winapi = lua.create_table()?;

    let find_window = lua.create_function(|_, query: String| {
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
    globals.set("find_window", find_window)?;

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
    globals.set("find_windows", find_windows)?;

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
    globals.set("get_window_title", get_window_title)?;


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
    globals.set("close_window", close_window)?;
    Ok(())
}
