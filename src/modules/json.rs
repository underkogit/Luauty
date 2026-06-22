use mlua::prelude::*;
use serde_json::{Map, Number, Value};

fn json_to_lua(lua: &Lua, value: &Value) -> LuaResult<LuaValue> {
    Ok(match value {
        Value::Null => LuaValue::Nil,
        Value::Bool(v) => LuaValue::Boolean(*v),
        Value::Number(v) => LuaValue::Number(v.as_f64().unwrap_or(0.0)),
        Value::String(v) => LuaValue::String(lua.create_string(v)?),
        Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, item)?)?;
            }
            LuaValue::Table(table)
        }
        Value::Object(obj) => {
            let table = lua.create_table()?;
            for (k, v) in obj {
                table.set(k.as_str(), json_to_lua(lua, v)?)?;
            }
            LuaValue::Table(table)
        }
    })
}

fn lua_to_json(value: LuaValue) -> LuaResult<Value> {
    match value {
        LuaValue::Nil => Ok(Value::Null),
        LuaValue::Boolean(v) => Ok(Value::Bool(v)),
        LuaValue::Integer(v) => Ok(Value::Number(Number::from(v))),
        LuaValue::Number(v) => Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| LuaError::RuntimeError("invalid number".into())),
        LuaValue::String(v) => Ok(Value::String(v.to_str()?.to_string())),
        LuaValue::Table(table) => {
            let mut is_array = true;
            let mut max_index = 0usize;
            let mut array_items = Vec::new();
            let mut object_items = Map::new();

            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (key, val) = pair?;
                match key {
                    LuaValue::Integer(i) if i > 0 => {
                        let idx = i as usize;
                        max_index = max_index.max(idx);
                        array_items.push((idx, lua_to_json(val)?));
                    }
                    LuaValue::Number(n) if n.fract() == 0.0 && n > 0.0 => {
                        let idx = n as usize;
                        max_index = max_index.max(idx);
                        array_items.push((idx, lua_to_json(val)?));
                    }
                    LuaValue::String(s) => {
                        is_array = false;
                        object_items.insert(s.to_str()?.to_string(), lua_to_json(val)?);
                    }
                    _ => is_array = false,
                }
            }

            if is_array {
                array_items.sort_by_key(|(i, _)| *i);
                if array_items.len() == max_index
                    && array_items
                        .iter()
                        .enumerate()
                        .all(|(n, (i, _))| *i == n + 1)
                {
                    Ok(Value::Array(
                        array_items.into_iter().map(|(_, v)| v).collect(),
                    ))
                } else {
                    let mut map = Map::new();
                    for (i, v) in array_items {
                        map.insert(i.to_string(), v);
                    }
                    Ok(Value::Object(map))
                }
            } else {
                Ok(Value::Object(object_items))
            }
        }
        _ => Err(LuaError::RuntimeError("unsupported Lua value".into())),
    }
}

pub fn init(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    let json_parse = lua.create_function(|lua, input: String| {
        let value: Value =
            serde_json::from_str(&input).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        json_to_lua(lua, &value)
    })?;
    globals.set("json_parse", json_parse)?;

    let json_stringify = lua.create_function(|_, value: LuaValue| {
        let json = lua_to_json(value)?;
        let text =
            serde_json::to_string(&json).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
        Ok(text)
    })?;
    globals.set("json_stringify", json_stringify)?;

    Ok(())
}
