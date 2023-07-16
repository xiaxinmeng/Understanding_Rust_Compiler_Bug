toml
[package]
name = "tiny"
version = "0.1.0"
authors = []
edition = "2018"

[profile.release]
opt-level = "s"
debug = false
rpath = false
lto = true
debug-assertions = false
codegen-units = 1
panic = 'abort'
incremental = false
