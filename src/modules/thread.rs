use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let sleep_fn = lua.create_function(|_, ms: u64| {
        thread::sleep(Duration::from_millis(ms));
        Ok(true)
    })?;
    globals.set("sleep", sleep_fn)?;

    Ok(())
}
