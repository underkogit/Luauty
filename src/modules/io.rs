use mlua::prelude::*;
use regex::Regex;
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

    let copy_file = lua.create_function(|_, (src, dst): (String, String)| {
        Ok(fs::copy(&src, &dst).is_ok())
    })?;

    let copy_dir = lua.create_function(|_, (src, dst): (String, String)| {
        Ok(copy_dir_all(Path::new(&src), Path::new(&dst)).is_ok())
    })?;

    let file_exists = lua.create_function(|_, path: String| {
        Ok(Path::new(&path).is_file())
    })?;

    let dir_exists = lua.create_function(|_, path: String| {
        Ok(Path::new(&path).is_dir())
    })?;

    let find_files_by_name = lua.create_function(|_, (dir_path, pattern): (String, String)| {
        let regex = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let mut results = Vec::new();
        find_files(Path::new(&dir_path), &regex, &mut results)?;
        Ok(results)
    })?;

    let find_file_by_name = lua.create_function(|_, (dir_path, pattern): (String, String)| {
        let regex = Regex::new(&pattern).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let mut results = Vec::new();
        find_files(Path::new(&dir_path), &regex, &mut results)?;
        Ok(results.into_iter().next())
    })?;

    globals.set("read_file", read_file)?;
    globals.set("write_file", write_file)?;
    globals.set("copy_file", copy_file)?;
    globals.set("copy_dir", copy_dir)?;
    globals.set("file_exists", file_exists)?;
    globals.set("dir_exists", dir_exists)?;
    globals.set("find_files_by_name", find_files_by_name)?;
    globals.set("find_file_by_name", find_file_by_name)?;

    Ok(())
}

fn find_files(dir: &Path, regex: &Regex, results: &mut Vec<String>) -> LuaResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir).map_err(|e| LuaError::RuntimeError(e.to_string()))? {
        let entry = entry.map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let path = entry.path();

        if path.is_dir() {
            find_files(&path, regex, results)?;
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if regex.is_match(name) {
                results.push(path.to_string_lossy().to_string());
            }
        }
    }

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