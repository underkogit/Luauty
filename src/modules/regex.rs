use mlua::prelude::*;
use regex::Regex;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    globals.set(
        "regex_is_match",
        lua.create_function(|_, (pattern, text): (String, String)| {
            let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(re.is_match(&text))
        })?,
    )?;

    globals.set(
        "regex_find",
        lua.create_function(|lua, (pattern, text): (String, String)| {
            let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            if let Some(m) = re.find(&text) {
                let t = lua.create_table()?;
                t.set("start", m.start() + 1)?;
                t.set("end", m.end())?;
                t.set("text", m.as_str())?;
                Ok(LuaValue::Table(t))
            } else {
                Ok(LuaValue::Nil)
            }
        })?,
    )?;

    globals.set(
        "regex_find_text",
        lua.create_function(|_, (pattern, text): (String, String)| {
            let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(re.find(&text).map(|m| m.as_str().to_string()))
        })?,
    )?;

    globals.set(
        "regex_replace",
        lua.create_function(
            |_, (pattern, text, replacement): (String, String, String)| {
                let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
                Ok(re.replace_all(&text, replacement.as_str()).to_string())
            },
        )?,
    )?;

    Ok(())
}
