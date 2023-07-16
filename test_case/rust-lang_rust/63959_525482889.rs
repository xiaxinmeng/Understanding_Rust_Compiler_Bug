toml
[package]
name = "cpu-bug"
version = "0.1.0"
authors = ["novacrazy <novacrazy@gmail.com>"]
edition = "2018"

[dependencies]
cargo_metadata = "0.8.2"

[profile.release] # My release profile
opt-level = 3
lto = 'fat'
incremental = false
debug-assertions = false
codegen-units = 1
