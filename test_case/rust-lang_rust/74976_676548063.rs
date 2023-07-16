toml
[build]
compiler-docs = true
extended = true
tools = [ "clippy", "rustfmt", "rust-analyzer" ]
target = [ "wasm32-unknown-unknown" ]

[rust]
incremental = true
lld = true
