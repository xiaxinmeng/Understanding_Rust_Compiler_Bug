toml
[package]
name = "itoatest"
version = "0.1.0"
authors = ["Steve Klabnik <steve@steveklabnik.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies.itoa]
version = "*"
features = ["std", "i128"]
