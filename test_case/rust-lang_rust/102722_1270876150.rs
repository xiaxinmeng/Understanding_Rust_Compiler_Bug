toml
# .cargo/config.coml
[target.mips-unknown-linux-gnu]
linker = "mips-linux-gnu-gcc"
runner = ["qemu-mips", "-L", "/usr/mips-linux-gnu", "-g", "23456"]
