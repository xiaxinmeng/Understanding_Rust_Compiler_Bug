 console
$ $(rustc +nightly --print sysroot)/bin/llvm-objdump -d target/riscv32imc-unknown-none-elf/release/libfoo.rlib
target/riscv32imc-unknown-none-elf/release/libfoo.rlib(foo-5c3d4a3cd43899d6.foo.axz7xdhy-cgu.0.rcgu.o): file format ELF32-riscv

Disassembly of section .text.foo:
foo:
       0:       41 11   addi    sp, sp, -16
       2:       06 c6   sw      ra, 12(sp)
       4:       95 45   addi    a1, zero, 5
       6:       97 00 00 00     auipc   ra, 0
       a:       e7 80 00 00     jalr    ra
       e:       b2 40   lw      ra, 12(sp)
      10:       41 01   addi    sp, sp, 16
      12:       82 80   ret

$(rustc +nightly --print sysroot)/bin/llvm-nm target/riscv32imc-unknown-none-elf/release/libfoo.rlib

foo-5c3d4a3cd43899d6.foo.axz7xdhy-cgu.0.rcgu.o:
         U __atomic_load_4
00000000 T foo
