toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

[profile.release]
lto = true
