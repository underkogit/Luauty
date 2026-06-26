use mlua::prelude::*;
use std::io::{self, Write};

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let console = lua.create_table()?;

    let read_line_fn = lua.create_function(|_, prompt: String| {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        Ok(input.trim_end().to_string())
    })?;
    console.set("read_line", read_line_fn)?;

    globals.set("console", console)?;

    Ok(())
}