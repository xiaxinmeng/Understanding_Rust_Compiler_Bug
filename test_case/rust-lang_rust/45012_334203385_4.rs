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
	movq	%rsi, %rbx
	movq	%rdi, %r14
	movl	$20, %esi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movl	8(%rbx), %eax
	movq	(%r14), %rcx
	movq	16(%r14), %rdx
	movl	%eax, (%rcx,%rdx)
	movl	12(%rbx), %eax
	movl	%eax, 4(%rcx,%rdx)
	movl	16(%rbx), %eax
	movl	%eax, 8(%rcx,%rdx)
	movq	(%rbx), %rax
	leaq	20(%rdx), %rsi
	movq	%rsi, 16(%r14)
	movq	%rax, 12(%rcx,%rdx)
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
	.cfi_endproc
