 console
$ $(rustc +nightly --print sysroot)/bin/llvm-objdump -d target/riscv32imac-unknown-none-elf/release/libfoo.rlib

target/riscv32imac-unknown-none-elf/release/libfoo.rlib(foo-5c3d4a3cd43899d6.foo.axz7xdhy-cgu.0.rcgu.o): file format ELF32-riscv

Disassembly of section .text.foo:
foo:
       0:       0f 00 30 03     fence   rw, rw
       4:       08 41   lw      a0, 0(a0)
       6:       0f 00 30 02     fence   r, rw
       a:       82 80   ret

$ $(rustc +nightly --print sysroot)/bin/llvm-nm target/riscv32imac-unknown-none-elf/release/libfoo.rlib

foo-5c3d4a3cd43899d6.foo.axz7xdhy-cgu.0.rcgu.o:
00000000 T foo
