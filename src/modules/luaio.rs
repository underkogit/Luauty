use mlua::prelude::*;
use std::fs;
use std::path::PathBuf;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let include = lua.create_function(|lua, path: String| {
        let script = fs::read_to_string(&path).map_err(|e| {
            eprintln!("include failed: {} ({})", path, e);
            LuaError::RuntimeError(e.to_string())
        })?;

        lua.load(&script).exec()?;
        Ok(())
    })?;

    let include_local = lua.create_function(|lua, path: String| {
        let mut full_path = PathBuf::from("scripts");
        full_path.push(&path);

        let script = fs::read_to_string(&full_path).map_err(|e| {
            eprintln!("include_local failed: {} ({})", full_path.display(), e);
            LuaError::RuntimeError(e.to_string())
        })?;

        lua.load(&script).exec()?;
        Ok(())
    })?;

    globals.set("include", include)?;
    globals.set("include_local", include_local)?;
    Ok(())
}
