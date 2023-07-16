toml
[unstable]
build-std = ["core", "alloc", "std"]

[build]
target = "x86_64-pc-windows-msvc"
rustflags = ["-Zshare-generics=y"]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[profile.release]
lto = "thin"
