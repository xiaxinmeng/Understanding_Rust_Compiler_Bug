toml
[package]
name = "asm_error"
version = "0.1.0"
edition = "2018"

# this causes a bug
[profile.release]
codegen-units = 1
lto = true
