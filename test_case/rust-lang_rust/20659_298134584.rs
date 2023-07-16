
test.o:     file format elf64-x86-64


Disassembly of section .text.start:

0000000000000000 <start>:
   0:	48 8b 3c 24          	mov    (%rsp),%rdi
   4:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
   9:	48 8b 05 00 00 00 00 	mov    0x0(%rip),%rax        # 10 <start+0x10>
  10:	48 89 38             	mov    %rdi,(%rax)
  13:	48 8b 05 00 00 00 00 	mov    0x0(%rip),%rax        # 1a <start+0x1a>
  1a:	48 89 30             	mov    %rsi,(%rax)
  1d:	48 8d 54 fe 08       	lea    0x8(%rsi,%rdi,8),%rdx
  22:	48 8b 05 00 00 00 00 	mov    0x0(%rip),%rax        # 29 <start+0x29>
  29:	48 89 10             	mov    %rdx,(%rax)
  2c:	48 8d 04 fe          	lea    (%rsi,%rdi,8),%rax
  30:	48 83 78 08 00       	cmpq   $0x0,0x8(%rax)
  35:	48 8d 40 08          	lea    0x8(%rax),%rax
  39:	75 f5                	jne    30 <start+0x30>
  3b:	50                   	push   %rax
  3c:	48 ff c8             	dec    %rax
  3f:	48 29 d0             	sub    %rdx,%rax
  42:	48 8b 0d 00 00 00 00 	mov    0x0(%rip),%rcx        # 49 <start+0x49>
  49:	48 89 01             	mov    %rax,(%rcx)
  4c:	e8 00 00 00 00       	callq  51 <start+0x51>
  51:	89 c7                	mov    %eax,%edi
  53:	e8 00 00 00 00       	callq  58 <start+0x58>
