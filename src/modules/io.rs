use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use mlua::prelude::*;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let read_file = lua.create_function(|_, path: String| {
        let mut file = File::open(&path).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(content)
    })?;
    globals.set("read_file", read_file)?;

    let write_file = lua.create_function(|_, (path, content): (String, String)| {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        file.write_all(content.as_bytes())
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        Ok(())
    })?;
    globals.set("write_file", write_file)?;

    Ok(())
}
