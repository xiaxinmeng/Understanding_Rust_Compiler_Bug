toml
# .cargo/config

[target.x86_64-unknown-linux-musl]
rustflags = [
    "-C", "target-feature=-crt-static",
]
