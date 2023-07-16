toml
[features]
parallel_compiler = ["indexmap/rustc-rayon", "rayon", "rayon-core"]

[dependencies]
indexmap = "1.8.0"
rayon = { version = "0.3.2", package = "rustc-rayon", optional = true }
rayon-core = { version = "0.3.2", package = "rustc-rayon-core", optional = true }
