#! /usr/bin/lua

-- Create release version for Linux (current os)
local handle_local_build = io.popen("cargo build --release")
if handle_local_build ~= nil then
  local result = handle_local_build:read("*a")
  print(result)
  handle_local_build:close()
end

-- add windows target
os.execute("rustup target add x86_64-pc-windows-gnu")

-- Create release version for Windows
local handle_win_build = io.popen("cargo build --release --target x86_64-pc-windows-gnu")
if handle_win_build ~= nil then
  local result = handle:read("*a")
  print(result)
  handle_win_build:close()
end

