use mlua::prelude::*;
use std::io::Read;
use std::process::{Command, Stdio};

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let run_process = lua.create_function(|lua, (program, args): (String, Vec<String>)| {
        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        let stdout =
            String::from_utf8(output.stdout).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let stderr =
            String::from_utf8(output.stderr).map_err(|e| LuaError::RuntimeError(e.to_string()))?;

        let result = lua.create_table()?;
        result.set("status", output.status.code())?;
        result.set("stdout", stdout)?;
        result.set("stderr", stderr)?;
        Ok(result)
    })?;

    let run_process_stream = lua.create_function(
        |lua, (program, args, callback): (String, Vec<String>, LuaFunction)| {
            let mut child = Command::new(program)
                .args(args)
                .stdout(Stdio::piped())
                .spawn()
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

            let mut stdout = child
                .stdout
                .take()
                .ok_or_else(|| LuaError::RuntimeError("stdout is not available".to_string()))?;

            let mut buffer = [0u8; 4096];
            let mut pending = Vec::new();
            let mut collected = String::new();

            loop {
                let n = stdout
                    .read(&mut buffer)
                    .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

                if n == 0 {
                    break;
                }

                pending.extend_from_slice(&buffer[..n]);

                while let Some(pos) = pending.iter().position(|&b| b == b'\n') {
                    let mut line = pending.drain(..=pos).collect::<Vec<u8>>();
                    if matches!(line.last(), Some(b'\n')) {
                        line.pop();
                    }
                    if matches!(line.last(), Some(b'\r')) {
                        line.pop();
                    }

                    let text = String::from_utf8_lossy(&line).into_owned();
                    callback.call::<()>(text.clone())?;
                    collected.push_str(&text);
                    collected.push('\n');
                }
            }

            if !pending.is_empty() {
                let text = String::from_utf8_lossy(&pending).into_owned();
                callback.call::<()>(text.clone())?;
                collected.push_str(&text);
            }

            let status = child
                .wait()
                .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

            let result = lua.create_table()?;
            result.set("status", status.code())?;
            result.set("stdout", collected)?;
            Ok(result)
        },
    )?;

    globals.set("run_process", run_process)?;
    globals.set("run_process_stream", run_process_stream)?;

    Ok(())
}
