
[package]
name = "repository"
version = "0.1.0"
edition = "2018"

[lib]
path = "src/lib.rs"
doctest = false

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
diesel = { version = "1.4.7", default-features = false, features = ["r2d2", "numeric", "chrono", "32-column-tables"] }
diesel-derive-enum = { version = "1.1.1", default-features = false }
diesel_migrations = "1.4.0"
futures-util = "0.3.15"
libsqlite3-sys = { version = "0.22.2", features = ["bundled"], optional = true }
serde = {version = "1.0.126", features = ["derive"]}
thiserror = "1"
log = "0.4.14"

[dev-dependencies]
actix-rt = "2.6.0"

[features]
default = ["sqlite"]
sqlite = ["diesel/sqlite", "libsqlite3-sys", "diesel-derive-enum/sqlite"]
memory = ["diesel/sqlite", "libsqlite3-sys", "diesel-derive-enum/sqlite"]
postgres = ["diesel/postgres" , "diesel-derive-enum/postgres"]
