toml
[dependencies]
log = { git = "https://github.com/rust-lang/log" }

[patch."https://github.com/rust-lang/log"]
log = { path = "./log" }
