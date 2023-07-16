toml
cargo-features = ["metabuild"]

[package]
metabuild = ["foo-bar"]

[build-dependencies.foo-bar]
path = "../build"
package = "my-build"
