toml
[package]
name = "repro"
version = "0.1.0"
edition = "2018"

[dependencies]
failure = "0.1.5"
serde = { version = "1.0.92", features = ["derive"] }
serde_json = "1.0.39"
chrono = { version = "0.4.6", features = ["serde"] }
tokio = "0.1.21"
reqwest = "0.9.18"
futures-preview = { version = "0.3.0-alpha.16", features = ["nightly", "async-await", "compat"] }
url = "1.7.2"
url_serde = "0.2.0"
