toml
[dependencies]
futures = { version = "0.3", default-features = false, features = ["alloc"] }
http = "0.2"
hyper = { version = "0.14", features = ["server", "http1", "tcp"] }
tokio = { version = "1.0", features = ["sync"] }
