toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=/STACK:0x800000"]
