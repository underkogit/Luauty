use mlua::prelude::*;

fn http_result(lua: &Lua, status: u16, content: String) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("status", status)?;
    table.set("content", content)?;
    Ok(table)
}

fn headers_to_reqwest(headers: LuaTable) -> LuaResult<reqwest::header::HeaderMap> {
    let mut map = reqwest::header::HeaderMap::new();

    for pair in headers.pairs::<String, String>() {
        let (key, value) = pair?;
        let name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        let val = reqwest::header::HeaderValue::from_str(&value)
            .map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        map.insert(name, val);
    }

    Ok(map)
}

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let http_get = lua.create_function(|lua, url: String| match reqwest::blocking::get(&url) {
        Ok(resp) => http_result(lua, resp.status().as_u16(), resp.text().unwrap_or_default()),
        Err(_) => http_result(lua, 0, String::new()),
    })?;

    let http_post = lua.create_function(|lua, (url, body): (String, String)| {
        let client = reqwest::blocking::Client::new();

        match client.post(&url).body(body).send() {
            Ok(resp) => http_result(lua, resp.status().as_u16(), resp.text().unwrap_or_default()),
            Err(_) => http_result(lua, 0, String::new()),
        }
    })?;

    let http_get_h = lua.create_function(|lua, (url, headers): (String, LuaTable)| {
        let client = reqwest::blocking::Client::new();
        let headers = headers_to_reqwest(headers)?;
        match client.get(&url).headers(headers).send() {
            Ok(resp) => http_result(lua, resp.status().as_u16(), resp.text().unwrap_or_default()),
            Err(_) => http_result(lua, 0, String::new()),
        }
    })?;

    let http_post_h =
        lua.create_function(|lua, (url, body, headers): (String, String, LuaTable)| {
            let client = reqwest::blocking::Client::new();
            let headers = headers_to_reqwest(headers)?;
            match client.post(&url).headers(headers).body(body).send() {
                Ok(resp) => {
                    http_result(lua, resp.status().as_u16(), resp.text().unwrap_or_default())
                }
                Err(_) => http_result(lua, 0, String::new()),
            }
        })?;

    globals.set("http_get", http_get)?;
    globals.set("http_post", http_post)?;
    globals.set("http_get_h", http_get_h)?;
    globals.set("http_post_h", http_post_h)?;
    Ok(())
}
