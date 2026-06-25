function printTable(t, indent)

	if type(t) ~= "table" then
		print(t)
		return
	end

	indent = indent or 0
	local spacing = string.rep("  ", indent)

	for k, v in pairs(t) do
		if type(v) == "table" then
			print(spacing .. tostring(k) .. ":")
			printTable(v, indent + 1)
		else
			print(spacing .. tostring(k) .. ":", v)
		end
	end
end