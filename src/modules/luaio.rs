use mlua::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();
    let base_dirs: Arc<RefCell<Vec<PathBuf>>> = Arc::new(RefCell::new(Vec::new()));
    let included_files: Arc<RefCell<HashSet<String>>> = Arc::new(RefCell::new(HashSet::new()));

    let user_dir = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

    let set_base_scripts = {
        let base_dirs = base_dirs.clone();
        lua.create_function(move |_, path: String| {
            let mut dir = PathBuf::from(path);
            if dir.is_relative() {
                dir = std::env::current_dir().map_err(|e| LuaError::RuntimeError(e.to_string()))?;
            }
            base_dirs.borrow_mut().push(dir);
            Ok(true)
        })?
    };

    let get_base_scripts = {
        let base_dirs = base_dirs.clone();
        lua.create_function(move |_, ()| {
            let paths: Vec<String> = base_dirs
                .borrow()
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            Ok(paths)
        })?
    };

    let get_included_files = {
        let included_files = included_files.clone();
        lua.create_function(move |_, ()| {
            let files: Vec<String> = included_files.borrow().iter().cloned().collect();
            Ok(files)
        })?
    };

    let include = {
        let included_files = included_files.clone();
        lua.create_function(move |lua, path: String| match fs::read_to_string(&path) {
            Ok(script) => {
                lua.load(&script).exec()?;
                included_files.borrow_mut().insert(path);
                Ok(true)
            }
            Err(_) => Ok(false),
        })?
    };

    let include_local = {
        let base_dirs = base_dirs.clone();
        let included_files = included_files.clone();
        lua.create_function(move |lua, path: String| {
            let dirs = base_dirs.borrow();

            for base_path in dirs.iter() {
                let full_path = base_path.join(&path);
                let path_str = full_path.to_string_lossy().to_string();

                if let Ok(script) = fs::read_to_string(&full_path) {
                    lua.load(&script).exec()?;
                    included_files.borrow_mut().insert(path_str);
                    return Ok(true);
                }
            }

            if let Ok(current_dir) = std::env::current_dir() {
                let full_path = current_dir.join(&path);
                let path_str = full_path.to_string_lossy().to_string();
                if let Ok(script) = fs::read_to_string(&full_path) {
                    lua.load(&script).exec()?;
                    included_files.borrow_mut().insert(path_str);
                    return Ok(true);
                }
            }

            Ok(false)
        })?
    };

    let include_dir = {
        let included_files = included_files.clone();
        lua.create_function(move |lua, path: String| {
            let dir = PathBuf::from(&path);
            if !dir.is_dir() {
                return Ok(0);
            }

            let mut loaded = 0;
            for entry in fs::read_dir(&dir).map_err(|e| LuaError::RuntimeError(e.to_string()))? {
                let entry = entry.map_err(|e| LuaError::RuntimeError(e.to_string()))?;
                let file_path = entry.path();

                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "luau" || ext == "lua" {
                            if let Ok(script) = fs::read_to_string(&file_path) {
                                if lua.load(&script).exec().is_ok() {
                                    included_files
                                        .borrow_mut()
                                        .insert(file_path.to_string_lossy().to_string());
                                    loaded += 1;
                                }
                            }
                        }
                    }
                }
            }

            Ok(loaded)
        })?
    };

    globals.set("USER_DIR", user_dir)?;
    globals.set("SET_BASE_SCRIPTS", set_base_scripts)?;
    globals.set("GET_BASE_SCRIPTS", get_base_scripts)?;
    globals.set("get_included_files", get_included_files)?;
    globals.set("include", include)?;
    globals.set("include_local", include_local)?;
    globals.set("include_dir", include_dir)?;

    Ok(())
}
