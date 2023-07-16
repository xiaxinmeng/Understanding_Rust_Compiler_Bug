 toml
[package]

[dependencies.alloc_system]
path = "$RUST_SRC/src/liballoc_system"

[dependencies.panic_abort]
path = "$RUST_SRC/src/libpanic_abort"

[dependencies.std]
path = "$RUST_SRC/src/libstd"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
