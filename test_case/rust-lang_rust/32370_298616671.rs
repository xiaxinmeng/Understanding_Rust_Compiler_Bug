asm
Disassembly of section .text._ZN4test6foobar17he9df46e58c7bef82E:

0000000000000000 <_ZN4test6foobar17he9df46e58c7bef82E>:
   0:	31 c0                	xor    %eax,%eax
   2:	80 7f 03 2a          	cmpb   $0x2a,0x3(%rdi)
   6:	0f b6 0f             	movzbl (%rdi),%ecx
   9:	66 0f 45 c1          	cmovne %cx,%ax
   d:	c3                   	retq
