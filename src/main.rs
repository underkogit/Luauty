mod modules;

use mlua::prelude::*;
use std::fs;
use std::io::{self, Write};

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    modules::luaio::init(&lua)?;
    modules::winapi::init(&lua)?;
    modules::io::init(&lua)?;
    modules::json::init(&lua)?;
    modules::http::init(&lua)?;
    let script = fs::read_to_string("scripts\\init.luau")
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

    lua.load(&script).exec()?;

    Ok(())
}
