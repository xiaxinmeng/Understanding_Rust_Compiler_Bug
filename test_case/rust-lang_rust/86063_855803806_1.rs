toml
[package]
name = "mcve"
version = "0.1.0"
edition = "2018"

[profile.dev]
codegen-units = 1
opt-level = 1

[profile.release]
codegen-units = 1
debug = true
lto = true

[dependencies]
cortex-m-semihosting = "0.3.3"
cortex-m-rt = "0.6.14"
