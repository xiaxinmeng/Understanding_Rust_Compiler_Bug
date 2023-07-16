asm
	.def	swap_strings;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_strings
	.globl	swap_strings
	.p2align	4, 0x90
swap_strings:
	movups	(%rdx), %xmm0
	movups	(%rcx), %xmm1
	movups	%xmm0, (%rcx)
	movups	%xmm1, (%rdx)
	movq	16(%rcx), %rax
	movq	16(%rdx), %r8
	movq	%r8, 16(%rcx)
	movq	%rax, 16(%rdx)
	retq

	.def	swap_another;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_another
	.globl	swap_another
	.p2align	4, 0x90
swap_another:
	movq	(%rcx), %rax
	movq	(%rdx), %r8
	movq	%r8, (%rcx)
	movq	%rax, (%rdx)
	movl	8(%rcx), %eax
	movl	8(%rdx), %r8d
	movl	%r8d, 8(%rcx)
	movl	%eax, 8(%rdx)
	retq
