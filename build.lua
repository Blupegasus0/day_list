#! /usr/bin/lua

Dependencies_linux = {
  "cargp",
  "rustup",
  "mingw-w64",
}

-- Create release version for Linux (current os)
local local_build = "cargo build --release"
local handle_local_build = io.popen(local_build)
print("Running: " .. local_build)
if handle_local_build ~= nil then
  local result = handle_local_build:read("*a")
  local success, _, exit_code = handle_local_build:close()
  if not success then
    print("error: " .. exit_code .. "\n" .. result)
  end
end

-- add windows target
local add_win_target = "rustup target add x86_64-pc-windows-gnu"
local handle_add_target= io.popen(add_win_target)
print("Running: " .. add_win_target)
if handle_add_target ~= nil then
  local result = handle_add_target:read("*a")
  local success, _, exit_code = handle_add_target:close()
  if not success then
    print("error: " .. exit_code .. "\n" .. result)
  end
end

-- Create release version for Windows
local win_build = "cargo build --release --target x86_64-pc-windows-gnu"
local handle_win_build = io.popen(win_build)
print("Running: " .. win_build)
if handle_win_build ~= nil then
  local result = handle_win_build:read("*a")
  local success, _, exit_code = handle_win_build:close()
  if not success then
    print("error: " .. exit_code .. "\n" .. result)
  end
end

