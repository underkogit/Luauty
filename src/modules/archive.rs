use mlua::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use walkdir::WalkDir;
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

 
    let pack_folder = lua.create_function(|_, (src, dst, options): (String, String, Option<LuaTable>)| {
        let mut ignore_list = Vec::new();
        let mut compression_level = 9; // Максимальное сжатие по умолчанию

        if let Some(opts) = options {
            // Получаем список игнорирования
            if let Ok(ignore) = opts.get::<Vec<String>>("ignore") {
                ignore_list = ignore;
            }

            // Получаем уровень сжатия (0-9)
            if let Ok(level) = opts.get::<u32>("compression") {
                compression_level = level.min(9);
            }

            // Проверяем флаг максимального сжатия
            if let Ok(max_compression) = opts.get::<bool>("max_compression") {
                if max_compression {
                    compression_level = 9;
                }
            }
        }

        match pack_directory(Path::new(&src), &dst, &ignore_list, compression_level) {
            Ok(path) => Ok(path),
            Err(e) => Err(LuaError::RuntimeError(e.to_string())),
        }
    })?;

    // Распаковка архива
    let unpack_archive = lua.create_function(|_, (archive_path, dst): (String, String)| {
        match unpack_tar_gz(Path::new(&archive_path), Path::new(&dst)) {
            Ok(_) => Ok(true),
            Err(e) => Err(LuaError::RuntimeError(e.to_string())),
        }
    })?;

    // Получение информации об архиве - используем Lua::clone
    let archive_info = {
        let lua_clone = lua.clone();
        lua.create_function(move |_, archive_path: String| {
            match get_archive_info(&lua_clone, Path::new(&archive_path)) {
                Ok(info) => Ok(info),
                Err(e) => Err(LuaError::RuntimeError(e.to_string())),
            }
        })?
    };

    // Список файлов в архиве
    let list_archive = lua.create_function(|_, archive_path: String| {
        match list_archive_contents(Path::new(&archive_path)) {
            Ok(files) => Ok(files),
            Err(e) => Err(LuaError::RuntimeError(e.to_string())),
        }
    })?;

    globals.set("pack_folder", pack_folder)?;
    globals.set("unpack_archive", unpack_archive)?;
    globals.set("archive_info", archive_info)?;
    globals.set("list_archive", list_archive)?;

    Ok(())
}

/// Упаковка директории в tar.gz архив
fn pack_directory(
    src: &Path,
    dst: &str,
    ignore_list: &[String],
    compression_level: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    if !src.exists() {
        return Err(format!("Source directory '{}' does not exist", src.display()).into());
    }

    if !src.is_dir() {
        return Err(format!("'{}' is not a directory", src.display()).into());
    }

    // Создаем файл архива
    let dst_path = if dst.ends_with(".tar.gz") || dst.ends_with(".tgz") {
        dst.to_string()
    } else {
        format!("{}.tar.gz", dst)
    };

    let file = File::create(&dst_path)?;
    let buf_writer = BufWriter::new(file);

    // Создаем GZIP компрессор с указанным уровнем сжатия
    let compression = match compression_level {
        0 => Compression::none(),
        1..=3 => Compression::fast(),
        4..=6 => Compression::default(),
        7..=9 => Compression::best(),
        _ => Compression::best(),
    };

    let gz_encoder = GzEncoder::new(buf_writer, compression);
    let mut tar_builder = Builder::new(gz_encoder);

    // Обходим все файлы в директории
    for entry in WalkDir::new(src)
        .into_iter()
        .filter_entry(|e| !should_ignore(e.path(), ignore_list))
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(src)?;

            // Добавляем файл в архив с метаданными
            tar_builder.append_file(relative_path, &mut File::open(path)?)?;
        }
    }

    // Завершаем создание архива
    tar_builder.finish()?;

    Ok(dst_path)
}

/// Проверка, нужно ли игнорировать файл/папку
fn should_ignore(path: &Path, ignore_list: &[String]) -> bool {
    if ignore_list.is_empty() {
        return false;
    }

    let path_str = path.to_string_lossy();
    let file_name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();

    for pattern in ignore_list {
        // Точное совпадение с именем файла/папки
        if pattern.as_str() == file_name {
            return true;
        }

        // Частичное совпадение в пути
        if path_str.contains(pattern.as_str()) {
            return true;
        }

        // Проверка расширения
        if let Some(ext) = path.extension() {
            let ext_str = format!(".{}", ext.to_string_lossy());
            if pattern == &ext_str {
                return true;
            }
        }
    }

    false
}

/// Распаковка tar.gz архива
fn unpack_tar_gz(archive_path: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !archive_path.exists() {
        return Err(format!("Archive '{}' does not exist", archive_path.display()).into());
    }

    // Создаем целевую директорию
    fs::create_dir_all(dst)?;

    // Открываем архив
    let file = File::open(archive_path)?;
    let buf_reader = BufReader::new(file);
    let gz_decoder = flate2::read::GzDecoder::new(buf_reader);
    let mut tar_archive = tar::Archive::new(gz_decoder);

    // Распаковываем
    tar_archive.unpack(dst)?;

    Ok(())
}

/// Получение информации об архиве
fn get_archive_info(lua: &Lua, archive_path: &Path) -> Result<LuaTable, Box<dyn std::error::Error>> {
    if !archive_path.exists() {
        return Err(format!("Archive '{}' does not exist", archive_path.display()).into());
    }

    let metadata = fs::metadata(archive_path)?;
    let file_size = metadata.len();

    // Открываем архив для подсчета файлов
    let file = File::open(archive_path)?;
    let buf_reader = BufReader::new(file);
    let gz_decoder = flate2::read::GzDecoder::new(buf_reader);
    let mut tar_archive = tar::Archive::new(gz_decoder);

    let mut total_files = 0;
    let mut total_uncompressed_size = 0;

    for entry in tar_archive.entries()? {
        let entry = entry?;
        total_files += 1;
        total_uncompressed_size += entry.header().size()?;
    }

    // Создаем Lua таблицу с информацией
    let table = lua.create_table()?;

    table.set("file_size", file_size)?;
    table.set("total_files", total_files)?;
    table.set("uncompressed_size", total_uncompressed_size)?;
    table.set("compression_ratio", (total_uncompressed_size as f64 / file_size as f64))?;
    table.set("name", archive_path.file_name().unwrap_or_default().to_string_lossy())?;
    table.set("path", archive_path.to_string_lossy())?;

    Ok(table)
}

/// Список файлов в архиве
fn list_archive_contents(archive_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !archive_path.exists() {
        return Err(format!("Archive '{}' does not exist", archive_path.display()).into());
    }

    let file = File::open(archive_path)?;
    let buf_reader = BufReader::new(file);
    let gz_decoder = flate2::read::GzDecoder::new(buf_reader);
    let mut tar_archive = tar::Archive::new(gz_decoder);

    let mut files = Vec::new();

    for entry in tar_archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        files.push(path.to_string_lossy().to_string());
    }

    Ok(files)
}