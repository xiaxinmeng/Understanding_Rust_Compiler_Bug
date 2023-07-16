
[package]
name = "regress"
version = "0.1.0"
authors = ["Piotr Sikora <piotrsikora@google.com>"]
edition = "2018"

[profile.release]
lto = true
opt-level = 3
panic = "abort"
