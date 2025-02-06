local hello = "Hello, finally this is nvim"
local proof = vim.inspect({
	one = "This is a string",
	two = 123,
	three = function()
		return "hello"
	end,
	four = require,
	five = require
})

vim.print(hello)
vim.print(vim.version())
vim.print(proof)
vim.print("args: ".. vim.inspect{...})
vim.print("special: "..vim.inspect(special))
vim.print("ffi exists "..vim.inspect(package.loaded.ffi ~= nil))
