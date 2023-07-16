
(gdb) disassemble
Dump of assembler code for function _ZN6rust294main17h0036c8f76f3b37ddE:
   0x0000555555559160 <+0>:	push   %rax
   0x0000555555559161 <+1>:	movabs $0x55555555916b,%rax
=> 0x000055555555916b <+11>:	jmp    *%rax
   0x000055555555916d <+13>:	pop    %rax
   0x000055555555916e <+14>:	ret
End of assembler dump.
(gdb) p/x $rax
$3 = 0x55555555916b
