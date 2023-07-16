
[package]
name = "lenrs"
version = "0.1.0"
edition = "2018"

[dependencies.pyo3]
version = "0.4"
features = ["extension-module"]

[lib]
crate-type = ["cdylib"]
name = "lenrs"
