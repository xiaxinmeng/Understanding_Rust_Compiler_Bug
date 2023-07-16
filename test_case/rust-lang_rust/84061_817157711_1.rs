asm
	.def	 method_b;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,method_b
	.globl	method_b
	.p2align	4, 0x90
method_b:
	retq
