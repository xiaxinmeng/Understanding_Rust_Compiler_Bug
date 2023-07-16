toml
[package]
name = "rust-71504"
version = "0.1.0"
authors = ["Vincent Rouillé <vincent@speedy37.fr>"]
edition = "2018"

[dependencies]
globset = "0.4.5"

[profile.release]
lto = "thin"
