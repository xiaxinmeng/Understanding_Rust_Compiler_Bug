asm
00000000400855ec <volatile_read_u32>:
    400855ec:	b9410800 	ldr	w0, [x0, #264]
    400855f0:	d65f03c0 	ret

Disassembly of section .text.volatile_read:

00000000400855f4 <volatile_read>:
    400855f4:	b9410c00 	ldr	w0, [x0, #268]
    400855f8:	d65f03c0 	ret

Disassembly of section .text.volatile_write:

00000000400855fc <volatile_write>:
    400855fc:	52800028 	mov	w8, #0x1                   	// #1
    40085600:	b9011008 	str	w8, [x0, #272]
    40085604:	d65f03c0 	ret
