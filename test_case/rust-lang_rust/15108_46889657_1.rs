
$ objdump -S foo.o

foo.o:     file format elf64-x86-64


Disassembly of section .text._ZN4main20h22fc21a23bd4f614eaa4v0.0E:

0000000000000000 <_ZN4main20h22fc21a23bd4f614eaa4v0.0E>:
   0:   c3                      retq   

Disassembly of section .text.main:

0000000000000000 <main>:
  10:   48 89 f0                mov    %rax,%rdx
  13:   48 89 f9 48 8d          jmpq   18 <main+0x8>
  18:   Address 0x0000000000000018 is out of bounds.
