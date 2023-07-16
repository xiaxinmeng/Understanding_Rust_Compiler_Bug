
[package]
name = "panicbug"
version = "0.0.0"
edition = "2021"

[lib]
name = "panicbug"
crate-type = ["staticlib"]

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[dependencies]
