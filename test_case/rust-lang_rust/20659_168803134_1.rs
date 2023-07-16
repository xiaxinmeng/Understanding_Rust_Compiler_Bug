 text
test.o:     file format elf64-x86-64


Disassembly of section .text.start:

0000000000000000 <start>:
   0:   50                      push   %rax
   1:   48 8b 3c 24             mov    (%rsp),%rdi
   5:   48 8d 74 24 08          lea    0x8(%rsp),%rsi
   a:   48 8b 05 00 00 00 00    mov    0x0(%rip),%rax        # 11 <start+0x11>
  11:   48 89 38                mov    %rdi,(%rax)
  14:   48 8b 05 00 00 00 00    mov    0x0(%rip),%rax        # 1b <start+0x1b>
  1b:   48 89 30                mov    %rsi,(%rax)
  1e:   48 8d 54 fe 08          lea    0x8(%rsi,%rdi,8),%rdx
  23:   48 8b 05 00 00 00 00    mov    0x0(%rip),%rax        # 2a <start+0x2a>
  2a:   48 89 10                mov    %rdx,(%rax)
  2d:   48 8d 04 fe             lea    (%rsi,%rdi,8),%rax
  31:   66 66 66 66 66 66 2e    data16 data16 data16 data16 data16 nopw %cs:0x0(%rax,%rax,1)
  38:   0f 1f 84 00 00 00 00 
  3f:   00 
  40:   48 83 78 08 00          cmpq   $0x0,0x8(%rax)
  45:   48 8d 40 08             lea    0x8(%rax),%rax
  49:   75 f5                   jne    40 <start+0x40>
  4b:   48 ff c8                dec    %rax
  4e:   48 29 d0                sub    %rdx,%rax
  51:   48 8b 0d 00 00 00 00    mov    0x0(%rip),%rcx        # 58 <start+0x58>
  58:   48 89 01                mov    %rax,(%rcx)
  5b:   e8 00 00 00 00          callq  60 <start+0x60>
  60:   89 c7                   mov    %eax,%edi
  62:   e8 00 00 00 00          callq  67 <start+0x67>

