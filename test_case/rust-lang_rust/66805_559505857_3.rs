toml
[package]
name = "pm"
version = "0.0.0"
edition = "2018"

[lib]
proc-macro = true

[dependencies]
syn = { version = "1", features = ["full", "visit-mut"]}
quote = "1"
