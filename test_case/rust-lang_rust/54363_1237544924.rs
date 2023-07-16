toml
# Cargo.toml

[package]
name = "serde_derive"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1"
quote = "1"
syn = "1"

[build-dependencies]
autocfg = "1"

[dev-dependencies]
serde = "1"
trybuild = "1"

[macro-dependencies]
serde = "1"
