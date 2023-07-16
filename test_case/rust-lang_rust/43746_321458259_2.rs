asm
	.type	B,@object
	.section	.data.rel.ro.B,"aw",@progbits
	.globl	B
	.p2align	3
B:
	.quad	A
	.size	B, 8
