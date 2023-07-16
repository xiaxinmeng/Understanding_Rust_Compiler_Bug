toml
[package]
name = "recovery_netstack"
version = "0.0.0"

[dependencies]
zerocopy = { path = "../../public/rust/crates/zerocopy" }
byteorder = { version = "1.2.6", features = ["i128"] }
failure = "*"
rand = "0.4"
