
[build]
vendor = true
docs = false
sanitizers = true
rustc = "<path to beta toolchain>/bin/rustc"
cargo = "<path to beta toolchain>/bin/cargo"
rustfmt = "<path to beta toolchain>/bin/rustfmt"
target = ["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"]
