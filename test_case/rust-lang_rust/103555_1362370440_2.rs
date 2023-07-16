toml
[dev-dependencies]
criterion = { version = "0.4.0", features = ["html_reports"] }
rand = { version = "0.8.5", features = ["min_const_gen"] }

[[bench]]
name = "bench"
path = "src/bench.rs"
harness = false
