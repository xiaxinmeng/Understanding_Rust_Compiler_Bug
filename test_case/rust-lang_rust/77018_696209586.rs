toml
[package]
name = "example"
version = "0.1.0"
edition = "2018"
build = "build.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = { version = "2.0.0", default-features = false, features = ["rustls", "compress"] }
env_logger = "0.7.1"
log = "0.4.8"
thiserror = "1.0.15"
actix-web-actors = "2.0.0"
actix-rt = "1.1.0"
clap = { version = "2.33.0", features = ["suggestions", "color"] }
lazy_static = "1.4.0"
actix = "0.9.0"
serde = { version = "1.0.90", features = ["derive"] }
serde_json = "1.0.39"
futures = { version = "0.3.4" }
backoff = "0.1.6"
rust-embed = { version = "5.6.0", features = ["compression"] }
mime_guess = "2.0.3"
tokio = { version = "0.2.13", features = ["net", "fs", "sync", "io-util", "rt-core", "blocking", "rt-threaded", "time"] }
actix-cors = "0.2.0"
uuid = { version = "0.8.1", features = ["v4"] }
elasticsearch = { version = "7.7.0-alpha.1", default-features = false, features = ["rustls-tls"] }
url = "2.1.1"
redis = "0.17.0"
reqwest = { version = "0.10.4", features = ["json", "stream", "rustls-tls", "gzip"] }
toml = "0.5.6"
diesel = { version = "1.4.2", features = ["r2d2", "postgres", "chrono", "uuid"] }
actix-web-httpauth = { version = "0.4.0", path = "third-party/actix-web-httpauth" }
r2d2 = "0.8.8"
derive_builder = "0.9.0"
chrono = { version = "0.4.15", features = ["serde"] }
tonic = {  git = "https://github.com/GopherJ/tonic", branch="consume-connect", features = ["transport"] }
prost = "0.6.1"
prost-types = "0.6.1"
prost-derive = "0.6.1"
semver = "0.9.0"
regex = "1.3.9"
rsmq_async = { git = "https://github.com/GopherJ/rsmq-async-rs", branch = "master" }
sendgrid = { git = "https://github.com/GopherJ/sendgrid-rs", branch = "master", features = ["async"] }

# make diesel being able to be compiled through musl
openssl = "0.10.29"
openssl-probe = "0.1.2"

[build-dependencies]
tonic-build = { version = "^0.3.0", features = ["prost"] }
chrono = "0.4.15"
