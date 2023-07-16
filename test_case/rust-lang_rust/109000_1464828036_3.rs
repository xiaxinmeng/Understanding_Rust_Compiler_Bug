toml
[package]
name = "avr-bug"
version = "0.1.0"
edition = "2021"

[dependencies]
panic-halt = "0.2.0"
ufmt = "0.1.0"
noble-secp256k1 = { git = "https://github.com/xphoniex/noble-secp256k1-rs", default-features = false, features = ["8-bit"] }
hmac-sha256 = { version = "1.1.6", default-features = false, features = ["opt_size"] }

[dependencies.arduino-hal]
git = "https://github.com/rahix/avr-hal"
rev = "4c9c44c314eb061ee20556ef10d45dea36e75ee4"
features = ["arduino-uno"]

[dependencies.avr-device]
version = "0.5.0"

[profile.dev]
panic = "abort"
debug = true
lto = true
opt-level = "s"

[profile.release]
panic = "abort"
codegen-units = 1
debug = true
lto = true
#opt-level = "s"
#opt-level = "z"
opt-level = 2
strip = true

[profile.dev.package.compiler_builtins]
overflow-checks = false
