use mlua::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

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

        Ok(true)
    })?;
    globals.set("write_file", write_file)?;

    let copy_file =
        lua.create_function(|_, (src, dst): (String, String)| Ok(fs::copy(&src, &dst).is_ok()))?;
    globals.set("copy_file", copy_file)?;

    let copy_dir = lua.create_function(|_, (src, dst): (String, String)| {
        Ok(copy_dir_all(Path::new(&src), Path::new(&dst)).is_ok())
    })?;
    globals.set("copy_dir", copy_dir)?;

    let file_exists = lua.create_function(|_, path: String| Ok(Path::new(&path).is_file()))?;
    globals.set("file_exists", file_exists)?;

    let dir_exists = lua.create_function(|_, path: String| Ok(Path::new(&path).is_dir()))?;
    globals.set("dir_exists", dir_exists)?;

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else if ty.is_file() {
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}
