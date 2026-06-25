--local url = "https://api.open-meteo.com/v1/forecast?latitude=55.7558&longitude=37.6173&current_weather=true"
--
--function regex_test()
--	local text = [[
--	A versatile and sturdy shelving unit designed for makers, workshops, and garages.
--	This project is perfect for organizing heavy tools, storing filament and materials, or securely housing your 3D printer.
--	]]
--
--
--
--
--	local regex_pattern = [[(organizing.+?or)]]
--	local is_match = regex_is_match(regex_pattern, text)
--	print(is_match)
--	___dbg_print_line()
--
--	local found_text = regex_find_text(regex_pattern, text)
--	print(found_text)
--	___dbg_print_line()
--
--	local regex_pattern_replace = [[((A versatile).+?\.)|(\n)]]
--	local replace_text = regex_replace(regex_pattern_replace, text , " ARA ARRRRAA ")
--	print(replace_text)
--	___dbg_print_line()
--end
--
--
--
--
----- Получить текущую погоду Open-Meteo для указанной точки.
--function get_open_meteo_weather()
--
--	local res = http_get(url)
--	if not res or res.status ~= 200 or res.content == "" then
--		return
--	end
--
--	local data = json_parse(res.content)
--	print(data.current_weather.temperature)
--end
--
----- Получить текущую погоду Open-Meteo с заголовками.
--function get_open_meteo_weather_with_headers()
--
--	local headers = {
--		["User-Agent"] = "WeatherApp/1.0",
--		["Accept"] = "application/json",
--	}
--
--	local res = http_get_h(url, headers)
--	if not res then
--		return
--	end
--
--	print(res.status)
--	print(res.content)
--end
--
----- Найти окна по подстроке в заголовке.
--function find_windows_test()
--	printTable(find_windows("Visual"))
--end
--
----- Закрыть процесс окна Блокнота.
--function close_notepad_process()
--	local hwnd: number = find_window("Без имени — Блокнот")
--	if hwnd == 0 then
--		return
--	end
--
--	local pid: number = get_window_process_id(hwnd)
--	print(hwnd)
--	print("PROC_ID:" .. tostring(pid))
--	close_process(pid)
--end
--
----- Проверить JSON-парсер и сериализацию.
--function test_json()
--	printTable(json_parse('{"name":"Alex"}'))
--
--	local data = {
--		a = 10,
--		b = 10,
--		data = {
--			a = 10,
--			b = 10,
--		},
--	}
--
--	print(json_stringify(data))
--end



function app_startup()



	local result = run_process("cmd", { "/C", "echo hello" }  )
	print(result.status)
	print(result.stdout)
	print(result.stderr)



	local result = run_process_stream("cmd", {
		"/C",
		"ping -n 2 google.com"
	}, function(stdout: string)
		print(stdout)
	end)

	print(result.status)

end


local function copy_file_and_dir()
	local file: string = "F:\\TorrentDownload\\cnc_gcodetest.gcode"
	local file_o: string = "F:\\TorrentDownload\\cnc_gcodetest_copy.gcode"

	if copy_file(file, file_o) then
		print("cool")
	end
	if file_exists(file) and file_exists(file_o) then
		print("file exists")
	end


end


@native
local function fib(n: number): number
	if n <= 1 then
		return n
	end
	return fib(n - 1) + fib(n - 2)
end


--type TTest<T> = {first: T , second: T}
--
--local testd : TTest<string> = {first = "asdasd" , second = "asdas12dasd"}
--
--printTable(testd)
--print(testd)



--local result: res.Result<number, string> = res.something()
--
--local t : Ok<string> = { type = "ok", value = "" }
--printTable( t)



--
--local Account = {}
--Account.__index = Account
--function Account.new(name, balance)
--	local self = {}
--	self.name = name
--	self.balance = balance
--
--	return setmetatable(self, Account)
--end
--
--function Account:deposit(credit)
--	self.balance += credit
--end
--
--
--function Account:withdraw(debit)
--	self.balance -= debit
--end
--
--local account = Account.new("Alexander", 500)
--
--local t = {10 ,10}
--local json_t = json_stringify(account)
--printTable(json_t)
--printTable(123)


