function open_meteo()
	local http_ug_api = "https://api.open-meteo.com/v1/forecast?latitude=55.7558&longitude=37.6173&current_weather=true"
	local http_result = http_get(http_ug_api);
	if(http_result) then

		print(http_result.status)
		--print(http_result.content)
		if http_result.status == 200 and not (http_result.content == "") then

			local result = json_parse(http_result.content)

			local dtemperature = result["current_weather"]["temperature"]
			print(dtemperature)
		--printTable(result)
		end
	end
end




function open_meteo_2()
	local headers = {
		["User-Agent"] = "WeatherApp/1.0",
		["Accept"] = "application/json"
	}

	local url = "https://api.open-meteo.com/v1/forecast?" ..
	"latitude=55.7558&longitude=37.6173&current_weather=true"

	local res = http_get_h(url, headers)
	print("Status:", res.status)
	print("Response:", res.content)
end

function winapi_test()
	local id_proc = find_windows("Visual")
	printTable(id_proc)
end



function json_parser()

	local data = json_parse('{"name":"Alex"}')
	printTable(data)
	local data2 = {
		a = 10,
		b = 10,
		data = {
			a = 10,
			b = 10
		}
	}
	local text = json_stringify(data2)
	print(text)
end


