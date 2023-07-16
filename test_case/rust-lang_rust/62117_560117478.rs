
sirius@localhost:~/rust-rv$ file bin/rustc
bin/rustc: ELF 64-bit LSB pie executable, UCB RISC-V, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-riscv64-lp64d.so.1, for GNU/Linux 4.15.0, with debug_info, not stripped
sirius@localhost:~/rust-rv$ qemu-riscv64 ./bin/rustc -Vv
rustc 1.41.0-nightly (930a0a2d0 2019-12-01)
binary: rustc
commit-hash: 930a0a2d0b9966b276de49e7a865186152c2f075
commit-date: 2019-12-01
host: riscv64gc-unknown-linux-gnu
release: 1.41.0-nightly
LLVM version: 9.0
sirius@localhost:~/rust-rv$ qemu-riscv64 ./bin/rustc -C linker=riscv64-unknown-linux-gnu-gcc -C opt-level=3 ~/hello.rs
sirius@localhost:~/rust-rv$ file ./hello
./hello: ELF 64-bit LSB pie executable, UCB RISC-V, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-riscv64-lp64d.so.1, for GNU/Linux 4.15.0, with debug_info, not stripped
sirius@localhost:~/rust-rv$ qemu-riscv64 ./hello
Hello, world!
