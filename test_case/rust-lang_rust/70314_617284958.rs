toml
[package]
name = "refinery-macros"
version = "0.2.0"
authors = []
edition = "2018"

[lib]
proc-macro = true

[dependencies]
quote = "1"
syn = { version = "1", features=["full"] }
proc-macro2 = "1"
