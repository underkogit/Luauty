use mlua::prelude::*;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, MB_OK, MB_OKCANCEL, MB_YESNO, MB_YESNOCANCEL,
    MB_ABORTRETRYIGNORE, MB_RETRYCANCEL, MB_CANCELTRYCONTINUE,
    MB_ICONINFORMATION, MB_ICONWARNING, MB_ICONERROR, MB_ICONQUESTION,
    MESSAGEBOX_RESULT, IDOK, IDCANCEL, IDABORT, IDRETRY, IDIGNORE,
    IDYES, IDNO, IDTRYAGAIN, IDCONTINUE
};

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let messagebox_fn = lua.create_function(|_, (text, title, buttons, icon): (String, Option<String>, Option<String>, Option<String>)| {
        let title_str = title.unwrap_or_else(|| "Message".to_string());

        let button_type = match buttons.as_deref() {
            Some("ok") | None => MB_OK,
            Some("okcancel") => MB_OKCANCEL,
            Some("yesno") => MB_YESNO,
            Some("yesnocancel") => MB_YESNOCANCEL,
            Some("abortretryignore") => MB_ABORTRETRYIGNORE,
            Some("retrycancel") => MB_RETRYCANCEL,
            Some("canceltrycontinue") => MB_CANCELTRYCONTINUE,
            _ => MB_OK,
        };

        let icon_type = match icon.as_deref() {
            Some("info") | None => MB_ICONINFORMATION,
            Some("warning") => MB_ICONWARNING,
            Some("error") => MB_ICONERROR,
            Some("question") => MB_ICONQUESTION,
            _ => MB_ICONINFORMATION,
        };

        let text_wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let title_wide: Vec<u16> = title_str.encode_utf16().chain(std::iter::once(0)).collect();

        let result = unsafe {
            MessageBoxW(
                None,
                PCWSTR::from_raw(text_wide.as_ptr()),
                PCWSTR::from_raw(title_wide.as_ptr()),
                button_type | icon_type,
            )
        };

        let result_str = match result {
            MESSAGEBOX_RESULT(v) if v == IDOK.0 => "ok",
            MESSAGEBOX_RESULT(v) if v == IDCANCEL.0 => "cancel",
            MESSAGEBOX_RESULT(v) if v == IDABORT.0 => "abort",
            MESSAGEBOX_RESULT(v) if v == IDRETRY.0 => "retry",
            MESSAGEBOX_RESULT(v) if v == IDIGNORE.0 => "ignore",
            MESSAGEBOX_RESULT(v) if v == IDYES.0 => "yes",
            MESSAGEBOX_RESULT(v) if v == IDNO.0 => "no",
            MESSAGEBOX_RESULT(v) if v == IDTRYAGAIN.0 => "tryagain",
            MESSAGEBOX_RESULT(v) if v == IDCONTINUE.0 => "continue",
            _ => "unknown",
        };

        Ok(result_str.to_string())
    })?;

    globals.set("messagebox", messagebox_fn)?;

    Ok(())
}