toml
[package]
name = "issue81278"
version = "0.1.0"
authors = []
edition = "2018"

[dependencies]
chrono = { version = "0.4.19", features = ["serde"] }
serde = { version = "1.0.120", features = ["derive"] }
rust_decimal = "1.9.0"
