toml
[package]
name = "testrust"
version = "0.1.0"
edition = "2018"

[dependencies]
rayon = "1.3.0"
num = "0.2.0"

[dependencies.ndarray]
version = "0.13.0"
features = ["rayon"]
