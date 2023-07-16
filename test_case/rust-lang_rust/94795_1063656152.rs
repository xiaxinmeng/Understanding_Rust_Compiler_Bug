toml
[profile.release]
debug = true
opt-level = 's'
lto = "thin"
codegen-units = 1
rpath = true
split-debuginfo = "packed"
