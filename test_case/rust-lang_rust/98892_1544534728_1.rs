asm
	.def	swap_strings;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_strings
	.globl	swap_strings
	.p2align	4, 0x90
swap_strings:
.seh_proc swap_strings
	subq	$24, %rsp
	.seh_stackalloc 24
	.seh_endprologue
	movq	16(%rcx), %rax
	movq	%rax, 16(%rsp)
	movups	(%rcx), %xmm0
	movaps	%xmm0, (%rsp)
	movq	16(%rdx), %rax
	movq	%rax, 16(%rcx)
	movups	(%rdx), %xmm0
	movups	%xmm0, (%rcx)
	movq	16(%rsp), %rax
	movq	%rax, 16(%rdx)
	movaps	(%rsp), %xmm0
	movups	%xmm0, (%rdx)
	addq	$24, %rsp
	retq
	.seh_endproc

	.def	swap_another;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_another
	.globl	swap_another
	.p2align	4, 0x90
swap_another:
.seh_proc swap_another
	subq	$16, %rsp
	.seh_stackalloc 16
	.seh_endprologue
	movl	8(%rcx), %eax
	movl	%eax, 8(%rsp)
	movq	(%rcx), %rax
	movq	%rax, (%rsp)
	movl	8(%rdx), %eax
	movl	%eax, 8(%rcx)
	movq	(%rdx), %rax
	movq	%rax, (%rcx)
	movl	8(%rsp), %eax
	movl	%eax, 8(%rdx)
	movq	(%rsp), %rax
	movq	%rax, (%rdx)
	addq	$16, %rsp
	retq
	.seh_endproc
