toml
[package]
name = "mypkg"
version = "0.1.0"
# Tell Cargo not to auto-scan for all tests/*.rs files.
autotests = false 

[[test]]
name = "mytest"
