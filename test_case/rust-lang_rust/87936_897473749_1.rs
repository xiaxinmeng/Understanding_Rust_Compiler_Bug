toml
[package]
name = "ice"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
ice-macros = { path = "../ice-macros"}
serde = { version = "1", features = ["derive"] }
