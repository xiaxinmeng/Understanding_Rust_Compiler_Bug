toml
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
rustflags = ["-Clink-self-contained=yes"]

[unstable]
build-std = ["core"]
