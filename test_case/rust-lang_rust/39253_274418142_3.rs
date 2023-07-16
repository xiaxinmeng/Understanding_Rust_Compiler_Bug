
$ objdump -Cd target/release/examples/app

target/release/examples/app:     file format elf64-x86-64


Disassembly of section .text:

00000000000002c0 <_start>:
 2c0:   48 8b 05 f9 ff ff ff    mov    -0x7(%rip),%rax        # 2c0 <_start>
