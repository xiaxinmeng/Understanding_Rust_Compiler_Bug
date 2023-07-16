toml
[target.riscv64gc-unknown-linux-gnu]
linker = "riscv64-linux-gnu-gcc"
rustflags = [
"-C", "target-feature=+crt-static",
]

[build]
# Set the default --target flag
target = "riscv64gc-unknown-linux-gnu"
