toml
[target.riscv64gc-unknown-linux-gnu]
    rustflags = [
        "-C", "link-arg=-L/usr/riscv64-linux-gnu/lib",
        "-C", "link-arg=-L/usr/riscv64-linux-gnu/usr/lib",
        "-C", "link-arg=-L/usr/lib/gcc/riscv64-linux-gnu/11.1.0/",
        "-C", "link-arg=--sysroot=/usr/riscv64-linux-gnu"
    ]

    linker = "rust-lld"
