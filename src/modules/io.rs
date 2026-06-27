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

    let copy_file =
        lua.create_function(|_, (src, dst): (String, String)| Ok(fs::copy(&src, &dst).is_ok()))?;

    let copy_dir = lua.create_function(
        |_, (src, dst, ignore_list): (String, String, Vec<String>)| match copy_dir_all(
            Path::new(&src),
            Path::new(&dst),
            &ignore_list,
        ) {
            Ok(_) => Ok(true),
            Err(e) => Err(LuaError::RuntimeError(e.to_string())),
        },
    )?;

    let file_exists = lua.create_function(|_, path: String| Ok(Path::new(&path).is_file()))?;

    let dir_exists = lua.create_function(|_, path: String| Ok(Path::new(&path).is_dir()))?;

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

    let get_parent_dir = lua.create_function(|_, path: String| {
        let path_obj = Path::new(&path);

        let parent = path_obj.parent();

        match parent {
            Some(dir) => {
                let dir_str = dir.to_string_lossy().to_string();

                if dir_str.is_empty() {
                    Ok(path)
                } else {
                    Ok(dir_str)
                }
            }
            None => Ok(path),
        }
    })?;
    let create_dir = lua.create_function(|_, path: String| {
        let path_obj = Path::new(&path);

        if path_obj.exists() {
            return Ok(false);
        }

        match fs::create_dir_all(path_obj) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    })?;
    globals.set("create_dir", create_dir)?;
    globals.set("get_parent_dir", get_parent_dir)?;
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
fn should_ignore(name: &str, ignore_list: &[String]) -> bool {
    ignore_list.iter().any(|pattern| {
        // Проверяем точное совпадение
        if pattern == name {
            return true;
        }
        // Проверяем, является ли имя файла/папки частью пути
        if name.contains(pattern) {
            return true;
        }
        false
    })
}

fn copy_dir_all(src: &Path, dst: &Path, ignore_list: &[String]) -> std::io::Result<()> {
    // Проверяем, что исходная директория существует
    if !src.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source directory '{}' does not exist", src.display()),
        ));
    }

    if !src.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("'{}' is not a directory", src.display()),
        ));
    }

    // Создаем целевую директорию
    fs::create_dir_all(dst)?;

    // Используем стек для обхода в глубину (итеративно)
    let mut stack = Vec::new();
    stack.push((src.to_path_buf(), dst.to_path_buf()));

    while let Some((from, to)) = stack.pop() {
        // Читаем содержимое директории
        let entries = match fs::read_dir(&from) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Warning: Cannot read directory '{}': {}", from.display(), e);
                continue;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Warning: Cannot read entry in '{}': {}", from.display(), e);
                    continue;
                }
            };

            let from_path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Проверяем, нужно ли игнорировать этот файл/папку
            if should_ignore(&file_name_str, ignore_list) {
                continue;
            }

            let to_path = to.join(&file_name);

            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    eprintln!(
                        "Warning: Cannot get file type for '{}': {}",
                        from_path.display(),
                        e
                    );
                    continue;
                }
            };

            if file_type.is_dir() {
                // Создаем поддиректорию и добавляем в стек для дальнейшего обхода
                fs::create_dir_all(&to_path)?;
                stack.push((from_path, to_path));
            } else if file_type.is_file() {
                // Копируем файл
                match fs::copy(&from_path, &to_path) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "Warning: Cannot copy file '{}' to '{}': {}",
                            from_path.display(),
                            to_path.display(),
                            e
                        );
                    }
                }
            } else if file_type.is_symlink() {
                // Копируем символическую ссылку (если поддерживается)
                #[cfg(unix)]
                {
                    match std::fs::read_link(&from_path) {
                        Ok(target) => {
                            if let Err(e) = std::os::unix::fs::symlink(target, &to_path) {
                                eprintln!(
                                    "Warning: Cannot create symlink '{}': {}",
                                    to_path.display(),
                                    e
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Cannot read symlink '{}': {}",
                                from_path.display(),
                                e
                            );
                        }
                    }
                }
                #[cfg(windows)]
                {
                    // На Windows просто копируем как файл
                    if let Err(e) = fs::copy(&from_path, &to_path) {
                        eprintln!(
                            "Warning: Cannot copy symlink '{}' to '{}': {}",
                            from_path.display(),
                            to_path.display(),
                            e
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
