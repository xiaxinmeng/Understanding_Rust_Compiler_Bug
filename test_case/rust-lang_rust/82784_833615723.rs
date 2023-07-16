toml
# Cargo.toml
[package]
name = "repro"
version = "0.1.0"
authors = []
edition = "2018"

[dependencies]
# The `macro-debug' branch contains a patch that prints the input and output of `tokio::main`:
# https://github.com/taiki-e/tokio/commit/13bdbe4b15183887b4382382dc7a6295155e533e
tokio = { version = "1", features = ["full"], git = "https://github.com/taiki-e/tokio.git", branch = "macro-debug" }
