# Lua Bindings Documentation

A comprehensive set of Lua bindings providing system-level functionality for scripting in Rust applications.

## Table of Contents
- [Overview](#overview)
- [HTTP Module](#http-module)
- [IO Module](#io-module)
- [JSON Module](#json-module)
- [Path Module](#path-module)
- [Process Module](#process-module)
- [Regex Module](#regex-module)
- [Thread Module](#thread-module)
- [WinAPI Module](#winapi-module)
- [LuaIO Module](#luaii-module)

---

## Overview

This library provides Lua bindings for various system operations including HTTP requests, file system operations, JSON parsing, process management, regex operations, and Windows API functions. All functions are globally available in the Lua environment.

---

## HTTP Module

### `http_get(url: string) -> table`

Performs a synchronous GET request.

**Parameters:**
- `url`: The target URL

**Returns:** Table with `status` (number) and `content` (string)

**Example:**
```lua
local result = http_get("https://api.example.com/data")
print("Status:", result.status)
print("Content:", result.content)
```

### `http_get_h(url: string, headers: table) -> table`

Performs a GET request with custom headers.

**Parameters:**
- `url`: The target URL
- `headers`: Table of header key-value pairs

**Returns:** Table with `status` (number) and `content` (string)

**Example:**
```lua
local headers = {
    ["Authorization"] = "Bearer token123",
    ["User-Agent"] = "MyApp/1.0"
}
local result = http_get_h("https://api.example.com/protected", headers)
print("Status:", result.status)
print("Content:", result.content)
```

### `http_post(url: string, body: string) -> table`

Performs a POST request with a raw body.

**Parameters:**
- `url`: The target URL
- `body`: Request body content

**Returns:** Table with `status` (number) and `content` (string)

**Example:**
```lua
local body = "Hello, Server!"
local result = http_post("https://httpbin.org/post", body)
print("Status:", result.status)
print("Response:", result.content)
```

### `http_post_h(url: string, body: string, headers: table) -> table`

Performs a POST request with custom headers.

**Parameters:**
- `url`: The target URL
- `body`: Request body content
- `headers`: Table of header key-value pairs

**Returns:** Table with `status` (number) and `content` (string)

**Example:**
```lua
local headers = {
    ["Content-Type"] = "application/json",
    ["X-Custom-Header"] = "Hello"
}
local body = '{"key": "value"}'
local result = http_post_h("https://httpbin.org/post", body, headers)
```

---

## IO Module

### `read_file(path: string) -> string`

Reads a file's contents as a string.

**Parameters:**
- `path`: File path

**Returns:** File content as string

**Example:**
```lua
local content = read_file("config.txt")
print(content)
```

### `write_file(path: string, content: string) -> boolean`

Writes content to a file (overwrites if exists).

**Parameters:**
- `path`: File path
- `content`: Content to write

**Returns:** `true` on success

**Example:**
```lua
write_file("output.txt", "Hello, World!")
```

### `copy_file(src: string, dst: string) -> boolean`

Copies a file.

**Parameters:**
- `src`: Source file path
- `dst`: Destination file path

**Returns:** `true` on success

**Example:**
```lua
copy_file("source.txt", "backup.txt")
```

### `copy_dir(src: string, dst: string) -> boolean`

Copies a directory recursively.

**Parameters:**
- `src`: Source directory path
- `dst`: Destination directory path

**Returns:** `true` on success

**Example:**
```lua
copy_dir("project/", "backup/")
```

### `file_exists(path: string) -> boolean`

Checks if a file exists.

**Parameters:**
- `path`: File path

**Returns:** `true` if file exists

**Example:**
```lua
if file_exists("data.json") then
    print("File exists")
end
```

### `dir_exists(path: string) -> boolean`

Checks if a directory exists.

**Parameters:**
- `path`: Directory path

**Returns:** `true` if directory exists

**Example:**
```lua
if dir_exists("logs/") then
    print("Directory exists")
end
```

---

## JSON Module

### `json_parse(json_str: string) -> table`

Parses a JSON string into a Lua table.

**Parameters:**
- `json_str`: JSON string

**Returns:** Lua table representation

**Example:**
```lua
local json = '{"name": "John", "age": 30, "hobbies": ["reading", "coding"]}'
local data = json_parse(json)
print(data.name)
print(data.age)
print(data.hobbies[1])
```

### `json_stringify(table: table) -> string`

Converts a Lua table to a JSON string.

**Parameters:**
- `table`: Lua table to convert

**Returns:** JSON string

**Example:**
```lua
local data = {
    name = "Jane",
    age = 25,
    skills = {"Lua", "Rust", "JSON"}
}
local json_str = json_stringify(data)
print(json_str)
```

---

## Path Module

### `join_path(a: string, b: string) -> string`

Joins two path components using the system's path separator.

**Parameters:**
- `a`: First path component
- `b`: Second path component

**Returns:** Joined path

**Example:**
```lua
local full_path = join_path("folder", "file.txt")
-- Returns "folder/file.txt" on Unix or "folder\\file.txt" on Windows
```

### `fix_path(path: string) -> string`

Normalizes a path by resolving `..` and `.` components.

**Parameters:**
- `path`: Path to normalize

**Returns:** Normalized path

**Example:**
```lua
local clean = fix_path("./folder/../other/./file.txt")
-- Returns "other/file.txt"
```

---

## Process Module

### `run_process(program: string, args: table) -> table`

Runs a program and waits for completion.

**Parameters:**
- `program`: Program to execute
- `args`: Table of command-line arguments

**Returns:** Table with `status` (number), `stdout` (string), and `stderr` (string)

**Example:**
```lua
local result = run_process("echo", {"Hello", "World"})
print("Status:", result.status)
print("Output:", result.stdout)
```

### `run_process_stream(program: string, args: table, callback: function) -> table`

Runs a program and processes output line by line.

**Parameters:**
- `program`: Program to execute
- `args`: Table of command-line arguments
- `callback`: Function called for each output line

**Returns:** Table with `status` (number) and `stdout` (string)

**Example:**
```lua
local callback = function(line)
    print("Line:", line)
end

local result = run_process_stream("ls", {"-la"}, callback)
print("Exit status:", result.status)
```

---

## Regex Module

### `regex_is_match(pattern: string, text: string) -> boolean`

Checks if text matches a regex pattern.

**Parameters:**
- `pattern`: Regular expression pattern
- `text`: Text to test

**Returns:** `true` if pattern matches

**Example:**
```lua
local has_number = regex_is_match("\\d+", "abc123def")
print(has_number) -- true
```

### `regex_find(pattern: string, text: string) -> table|nil`

Finds the first match and returns position information.

**Parameters:**
- `pattern`: Regular expression pattern
- `text`: Text to search

**Returns:** Table with `start`, `end`, and `text` fields, or `nil` if no match

**Example:**
```lua
local match = regex_find("\\d+", "abc123def")
if match then
    print("Match found at position:", match.start, "-", match["end"])
    print("Text:", match.text)
end
```

### `regex_find_text(pattern: string, text: string) -> string|nil`

Finds the first match and returns the matched text.

**Parameters:**
- `pattern`: Regular expression pattern
- `text`: Text to search

**Returns:** Matched text or `nil`

**Example:**
```lua
local number = regex_find_text("\\d+", "abc123def")
print(number) -- "123"
```

### `regex_replace(pattern: string, text: string, replacement: string) -> string`

Replaces all matches with a replacement string.

**Parameters:**
- `pattern`: Regular expression pattern
- `text`: Text to search
- `replacement`: Replacement text

**Returns:** Modified text

**Example:**
```lua
local result = regex_replace("\\d+", "abc123def456", "X")
print(result) -- "abcXdefX"
```

---

## Thread Module

### `sleep(ms: number) -> boolean`

Sleeps for a specified number of milliseconds.

**Parameters:**
- `ms`: Milliseconds to sleep

**Returns:** `true` on completion

**Example:**
```lua
print("Start")
sleep(2000) -- Wait 2 seconds
print("End")
```

---

## WinAPI Module (Windows only)

### `find_window_by_text(query: string) -> number`

Finds a window by its title (case-insensitive substring match).

**Parameters:**
- `query`: Text to search for in window titles

**Returns:** Window handle (HWND) or 0 if not found

**Example:**
```lua
local hwnd = find_window_by_text("Notepad")
if hwnd ~= 0 then
    print("Found Notepad window:", hwnd)
end
```

### `find_windows(query: string) -> table`

Finds all windows with titles containing the query.

**Parameters:**
- `query`: Text to search for in window titles

**Returns:** Table of window handles

**Example:**
```lua
local windows = find_windows("Notepad")
for i, hwnd in ipairs(windows) do
    print("Window:", hwnd)
end
```

### `get_window_title(hwnd: number) -> string`

Gets the title of a window.

**Parameters:**
- `hwnd`: Window handle

**Returns:** Window title

**Example:**
```lua
local hwnd = find_window_by_text("Notepad")
if hwnd ~= 0 then
    local title = get_window_title(hwnd)
    print("Title:", title)
end
```

### `get_window_process_id(hwnd: number) -> number`

Gets the process ID of a window.

**Parameters:**
- `hwnd`: Window handle

**Returns:** Process ID

**Example:**
```lua
local hwnd = find_window_by_text("Notepad")
if hwnd ~= 0 then
    local pid = get_window_process_id(hwnd)
    print("Process ID:", pid)
end
```

### `close_window(hwnd: number)`

Closes a window gracefully.

**Parameters:**
- `hwnd`: Window handle

**Example:**
```lua
local hwnd = find_window_by_text("Notepad")
if hwnd ~= 0 then
    close_window(hwnd)
end
```

### `close_process(pid: number)`

Terminates a process by ID.

**Parameters:**
- `pid`: Process ID

**Example:**
```lua
local hwnd = find_window_by_text("Notepad")
if hwnd ~= 0 then
    local pid = get_window_process_id(hwnd)
    close_process(pid)
end
```

---

## LuaIO Module

### `include(path: string)`

Loads and executes a Lua script from the given path.

**Parameters:**
- `path`: Path to the Lua script

**Example:**
```lua
include("lib/utils.lua")
```

### `include_local(path: string)`

Loads and executes a Lua script from the `scripts/` directory.

**Parameters:**
- `path`: Relative path within the `scripts/` directory

**Example:**
```lua
include_local("config.lua")
-- Loads "./scripts/config.lua"
```

---

## Practical Examples

### Web Scraper Example
```lua
-- Scrape a website and save data
local url = "https://api.github.com/repos/rust-lang/rust"
local result = http_get(url)

if result.status == 200 then
    local data = json_parse(result.content)
    print("Repository:", data.full_name)
    print("Stars:", data.stargazers_count)
    
    -- Save to file
    local json_str = json_stringify(data)
    write_file("rust_repo.json", json_str)
end
```

### Batch File Processor
```lua
-- Process all JSON files in a directory
function process_files()
    local files = {"data1.json", "data2.json", "config.json"}
    
    for _, file in ipairs(files) do
        if file_exists(file) then
            local content = read_file(file)
            local data = json_parse(content)
            
            -- Process data
            data.processed = true
            data.timestamp = os.time()
            
            -- Write back
            local updated = json_stringify(data)
            write_file(file, updated)
            print("Processed:", file)
        end
    end
end

process_files()
```

### System Monitor
```lua
-- Monitor system processes
local function monitor_process(process_name)
    local result = run_process("tasklist", {"/FI", "IMAGENAME eq " .. process_name})
    
    if string.find(result.stdout, process_name) then
        print(process_name, "is running")
    else
        print(process_name, "is not running")
    end
end

monitor_process("notepad.exe")
```

### HTTP API Client
```lua
-- Create an API client with authentication
local function api_request(endpoint, method, data)
    local base_url = "https://api.example.com"
    local headers = {
        ["Authorization"] = "Bearer your_token",
        ["Content-Type"] = "application/json"
    }
    
    if method == "GET" then
        return http_get_h(base_url .. endpoint, headers)
    elseif method == "POST" then
        local body = json_stringify(data)
        return http_post_h(base_url .. endpoint, body, headers)
    end
end

-- Usage
local result = api_request("/users/123", "GET")
if result.status == 200 then
    local user = json_parse(result.content)
    print("User:", user.name)
end
```

### File Backup System
```lua
-- Create a backup of important files
function backup_files()
    local backup_dir = "backup_" .. os.date("%Y%m%d")
    local files_to_backup = {"config.ini", "data.db", "settings.json"}
    
    if not dir_exists(backup_dir) then
        os.execute("mkdir " .. backup_dir)
    end
    
    for _, file in ipairs(files_to_backup) do
        if file_exists(file) then
            local dest = join_path(backup_dir, file)
            copy_file(file, dest)
            print("Backed up:", file)
        end
    end
end

backup_files()
```

---

## Notes

- All functions are synchronous and blocking
- HTTP functions use the blocking reqwest client
- Process functions capture stdout and stderr by default
- Windows-specific functions (WinAPI module) only work on Windows platforms
- File operations use the system's default encoding (UTF-8 recommended)
- Regex patterns use the Rust regex syntax
- JSON parsing supports all standard JSON types

---

## Error Handling

Most functions will return `nil` and print an error message on failure. You should check return values when appropriate:

```lua
-- Good practice
local content = read_file("file.txt")
if content then
    print("File read successfully")
else
    print("Failed to read file")
end

-- HTTP requests always return a table, check status
local result = http_get("https://example.com")
if result.status == 200 then
    print("Success:", result.content)
else
    print("HTTP error:", result.status)
end
```