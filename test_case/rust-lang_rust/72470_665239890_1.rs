toml
# Cargo.toml
[package]
name = "rust-template"
version = "0.1.0"
authors = ["linyongxing <xtutu0202@gmail.com>"]
edition = "2018"
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

default-run = "rust-template"

[dependencies]
tokio = { version = "=0.2.21", features = ["rt-core", "macros", "sync"] }
