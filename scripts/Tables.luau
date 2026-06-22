function printTable(t, indent)
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