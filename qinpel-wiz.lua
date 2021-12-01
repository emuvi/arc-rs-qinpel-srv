print("Building binary...")
wiz.cmd("cargo", {"build", "--release"}, ".", true, true)
local binary_name = "qinpel-srv" .. wiz.exe_ext
local binary_origin = "target/release/" .. binary_name
local binary_destiny = "../../" .. binary_name
local binary_old = binary_destiny .. "-old"
wiz.rm(binary_old)
wiz.mv(binary_destiny, binary_old)
wiz.cp(binary_origin, binary_destiny)
