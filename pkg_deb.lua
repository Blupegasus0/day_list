#! /usr/bin/lua
-- Lua script to create a .deb package for a Rust application

-- Define package metadata
local package_name = "my-rust-app"
local version = "1.0.0"
local maintainer = "Your Name <your.email@example.com>"
local description = "A simple to-do list app written in Rust."
local architecture = "amd64"
local dependencies = "libc6 (>= 2.17), libgcc1 (>= 1:3.0)" -- Adjust as needed
local binary_name = "my-rust-app"  -- The name of your compiled Rust binary

-- Define file paths
local base_dir = package_name .. "-" .. version
local debian_dir = base_dir .. "/DEBIAN"
local bin_dir = base_dir .. "/usr/local/bin"
local control_file = debian_dir .. "/control"
local target_binary_path = "target/release/" .. binary_name
local destination_binary_path = bin_dir .. "/" .. binary_name

-- Create directory structure
os.execute("mkdir -p " .. debian_dir)
os.execute("mkdir -p " .. bin_dir)

-- Write control file
local control_content = string.format([[
Package: %s
Version: %s
Section: utils
Priority: optional
Architecture: %s
Depends: %s
Maintainer: %s
Description: %s
]], package_name, version, architecture, dependencies, maintainer, description)

local control_file_handle = io.open(control_file, "w")
if control_file_handle ~= nil then
  control_file_handle:write(control_content)
  control_file_handle:close()
else
  error("error: failed to access control file", 1)
end

-- Copy the binary to the destination directory
os.execute("cp " .. target_binary_path .. " " .. destination_binary_path)

-- Set the correct permissions for the DEBIAN directory and its contents
os.execute("chmod -R 755 " .. debian_dir)

-- Build the .deb package
local package_filename = package_name .. "_" .. version .. "_" .. architecture .. ".deb"
os.execute("dpkg-deb --build " .. base_dir .. " " .. package_filename)

-- Clean up the temporary directory
os.execute("rm -rf " .. base_dir)

print("Package created: " .. package_filename)

