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
	movq	(%r14), %rax
	movq	16(%rbx), %rcx
	leaq	8(%rcx), %rdx
	movq	%rdx, 16(%rbx)
	movq	(%rbx), %rdx
	movq	%rax, (%rdx,%rcx)
	movq	8(%r14), %rax
	movq	16(%rbx), %rcx
	leaq	8(%rcx), %rdx
	movq	%rdx, 16(%rbx)
	movq	(%rbx), %rdx
	movq	%rax, (%rdx,%rcx)
	movq	16(%r14), %rax
	movq	16(%rbx), %rcx
	leaq	8(%rcx), %rdx
	movq	%rdx, 16(%rbx)
	movq	(%rbx), %rdx
	movq	%rax, (%rdx,%rcx)
	movq	24(%r14), %rax
	movq	16(%rbx), %rcx
	leaq	8(%rcx), %rdx
	movq	%rdx, 16(%rbx)
	movq	(%rbx), %rdx
	movq	%rax, (%rdx,%rcx)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
	.cfi_endproc
