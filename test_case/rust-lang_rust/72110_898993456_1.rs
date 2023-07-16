toml
cargo-features = ["strip"]

[package]
name = "hello"
version = "0.1.0"
edition = "2018"

[profile.release]
strip = "symbols"
