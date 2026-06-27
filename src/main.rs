mod modules;

use clap::Parser;
use mlua::prelude::*;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pub script: Option<String>,
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let args = Args::parse();
    let script_path = args.script.as_deref().unwrap_or("scripts\\main.luau");

    modules::console::init(&lua, script_path)?;
    for init in [
        modules::luaio::init as fn(&Lua) -> LuaResult<()>,
        modules::winapi::winapi::init,
        modules::winapi::winapi_messagebox::init,
        modules::io::init,
        modules::json::init,
        modules::http::init,
        modules::regex::init,
        modules::process::init,
        modules::thread::init,
        modules::path::init,
        modules::archive::init,
    ] {
        init(&lua)?;
    }

    let script =
        fs::read_to_string(script_path).map_err(|e| LuaError::RuntimeError(e.to_string()))?;

    lua.load(&script).exec()?;

    Ok(())
}
