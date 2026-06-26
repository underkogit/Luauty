use mlua::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let join_path = lua.create_function(|_, args: LuaMultiValue| {
        let mut base = PathBuf::new();
        let mut first = true;

        for arg in args.iter() {
            if let Ok(path_str) = arg.to_string() {
                if first {
                    base = PathBuf::from(path_str);
                    first = false;
                } else {
                    base = base.join(path_str);
                }
            }
        }

        Ok(base.to_string_lossy().into_owned())
    })?;

    let fix_path = lua.create_function(|_, path: String| {
        let mut out = PathBuf::new();

        for part in Path::new(&path).components() {
            match part {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    out.pop();
                }
                other => out.push(other.as_os_str()),
            }
        }

        Ok(out.to_string_lossy().into_owned())
    })?;

    let absolute_path = lua.create_function(|_, path: String| {
        let path = Path::new(&path);
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?
                .join(path)
        };

        let canonical = absolute
            .canonicalize()
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        Ok(canonical.to_string_lossy().into_owned())
    })?;

    globals.set("join_path", join_path)?;
    globals.set("fix_path", fix_path)?;
    globals.set("absolute_path", absolute_path)?;

    Ok(())
}
