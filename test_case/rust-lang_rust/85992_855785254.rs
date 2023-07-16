toml
[package]
name = "test_ice"
version = "0.1.0"
authors = ["username"]
edition = "2018"
publish = false

[dependencies]
diesel = { version = "1.4.6",   default-features = false }
diesel_migrations = { version = "1.4.0", default-features = false }
rocket = { version = "0.4.10",   default-features = false }
rocket_contrib = { version = "0.4.10",   default-features = false, features = ["diesel_postgres_pool"] }
