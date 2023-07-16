toml
build = "aarch64-apple-darwin" # because Rosetta confuses bootstrap
# Instead of downloading the src/stage0.json version of Cargo specified, use
# this Cargo binary instead to build all Rust code
cargo = "/path/to/cargo" # bypassing downloausds that might get the wrong compiler
