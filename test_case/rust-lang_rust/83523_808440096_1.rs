toml
[package]
name = "movei"
version = "0.1.0"
authors = ["..."]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rocket = "0.4.7"
serde = "1.0"
diesel = { version = "1.4", features = ["postgres"] }

[dependencies.serde_derive]
version = "1.0"

[dependencies.rocket_contrib]
version = "0.4"
default-features = false
features = ["json", "diesel_postgres_pool"]
