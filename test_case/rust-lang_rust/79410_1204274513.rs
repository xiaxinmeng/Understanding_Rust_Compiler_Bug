toml
[package]
name = "wasm-zero-address"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[profile.release]
debug = true
