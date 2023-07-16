toml
# Cargo.toml
[package]
name = "icy_project"
version = "0.1.0"
edition = "2018"

[dependencies]
approx="0.5.0"
simba="0.5.1"
# If you change to simba version 0.4, (which dependends on approx 0.4)
# then the test compilation ends with "error: cannot determine resolution for the import"

[dev-dependencies]
approx = "0.3.2"
alga = "0.9.3"

