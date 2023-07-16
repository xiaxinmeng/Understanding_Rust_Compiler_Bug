

[package]
name = "polargrid-server-shared"
version = "0.6.0"
authors = ["Jeroen Bakker <j.bakker@atmind.nl>"]
edition = "2018"

[lib]
name = "polargrid_server_shared"
path = "src/lib.rs"

[dependencies]
log = "0.4.5"
pretty_env_logger = "0.3.1"

lazy_static = "1.1.0"
time = "0.1.40"
clap = "2.32.0"
url = "1.7.1"
rust-ini = "0.13.0"
uuid = {version = "0.7", features = ["v4"]}
sha2 = "0.8.2"

maud = "0.22.0"

postgres = {version = "0.15.2", features = ["with-serde_json", "with-time"]}
postgres-derive = "0.3.3"

serde = "1.0"
serde_json = "1.0"
serde_derive = "1.0"

regex = "1.0.5"
http = "0.1.18"

hyper = "0.12.34"
mime = "0.3.14"

actix-web = "1.0.9"

polargrid-public-shared = { path="../polargrid-public-shared/"}

[profile.release]
lto = true
