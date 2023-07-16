toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang"]

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "linker=lld"]
