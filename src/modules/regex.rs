use mlua::prelude::*;
use regex::Regex;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let regex_is_match = lua.create_function(|_, (pattern, text): (String, String)| {
        let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(re.is_match(&text))
    })?;

    let regex_find = lua.create_function(|lua, (pattern, text): (String, String)| {
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
    })?;

    let regex_find_text = lua.create_function(|_, (pattern, text): (String, String)| {
        let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(re.find(&text).map(|m| m.as_str().to_string()))
    })?;

    let regex_replace = lua.create_function(
        |_, (pattern, text, replacement): (String, String, String)| {
            let re = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            Ok(re.replace_all(&text, replacement.as_str()).to_string())
        },
    )?;

    globals.set("regex_is_match", regex_is_match)?;
    globals.set("regex_find", regex_find)?;
    globals.set("regex_find_text", regex_find_text)?;
    globals.set("regex_replace", regex_replace)?;

    Ok(())
}
