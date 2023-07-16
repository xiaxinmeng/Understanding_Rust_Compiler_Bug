toml
[dependencies]
log = { git = "https://github.com/rust-lang/log" }

[patch.crates-io]
log = { path = "./log" }
