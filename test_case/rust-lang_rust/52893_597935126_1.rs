
[package]
name = "vtable"
version = "0.1.0"
authors = ["akiselev <code@akiselev.com>"]
edition = '2018'

[lib]
name = "vtable"

[dependencies]
frunk = "0.2.0"
frunk_core = { version = "0.2.0", features = ["serde"] }
failure = "0.1.1"
serde = "1.0.70"
serde_derive = "1.0.70"
serde_json = "1.0.24"
