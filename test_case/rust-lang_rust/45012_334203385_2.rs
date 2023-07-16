asm
__ZN10serde_fast10make_bytes17ha60b3205c9ed12fdE:
	.cfi_startproc
	pushq	%rbp
Lcfi71:
	.cfi_def_cfa_offset 16
Lcfi72:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi73:
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
Lcfi74:
	.cfi_offset %rbx, -32
Lcfi75:
	.cfi_offset %r14, -24
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movl	$32, %esi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movq	(%rbx), %rax
	movq	16(%rbx), %rcx
	movups	(%r14), %xmm0
	movups	%xmm0, (%rax,%rcx)
	movups	16(%r14), %xmm0
	leaq	32(%rcx), %rdx
	movq	%rdx, 16(%rbx)
	movups	%xmm0, 16(%rax,%rcx)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
	.cfi_endproc
