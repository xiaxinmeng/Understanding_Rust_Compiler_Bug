asm
__ZN10serde_fast15slow_make_bytes17hf60759a7955c0485E:
	.cfi_startproc
	pushq	%rbp
Lcfi76:
	.cfi_def_cfa_offset 16
Lcfi77:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi78:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	pushq	%rax
Lcfi79:
	.cfi_offset %rbx, -40
Lcfi80:
	.cfi_offset %r14, -32
Lcfi81:
	.cfi_offset %r15, -24
	movq	%rsi, %r14
	movq	%rdi, %rbx
	movq	(%r14), %r15
	movl	$8, %esi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movq	16(%rbx), %rax
	leaq	8(%rax), %rcx
	movq	%rcx, 16(%rbx)
	movq	(%rbx), %rcx
	movq	%r15, (%rcx,%rax)
	movq	8(%r14), %r15
	movl	$8, %esi
	movq	%rbx, %rdi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movq	16(%rbx), %rax
	leaq	8(%rax), %rcx
	movq	%rcx, 16(%rbx)
	movq	(%rbx), %rcx
	movq	%r15, (%rcx,%rax)
	movq	16(%r14), %r15
	movl	$8, %esi
	movq	%rbx, %rdi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movq	16(%rbx), %rax
	leaq	8(%rax), %rcx
	movq	%rcx, 16(%rbx)
	movq	(%rbx), %rcx
	movq	%r15, (%rcx,%rax)
	movq	24(%r14), %r14
	movl	$8, %esi
	movq	%rbx, %rdi
	callq	__ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$7reserve17hab984fb4b8ec6529E
	movq	16(%rbx), %rax
	leaq	8(%rax), %rcx
	movq	%rcx, 16(%rbx)
	movq	(%rbx), %rcx
	movq	%r14, (%rcx,%rax)
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
	.cfi_endproc
