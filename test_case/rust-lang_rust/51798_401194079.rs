
cargo-features = ["edition"]

[package]
publish = false
edition = "2018"
name = "rust-issue-51798-example-parent"
version = "1.0.0"

[dependencies]
rust-issue-51798-example-child = { version = "1.0.0", path = "child" }
