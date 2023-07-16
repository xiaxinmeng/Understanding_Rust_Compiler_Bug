
#Cargo.toml
[package]
authors = []
name = "logwatch"
version = "0.1.0"
edition = "2018"

[dependencies]
signal-hook = "0.1.8"
failure = "*"

[profile.release]
opt-level = "s"
lto = true
