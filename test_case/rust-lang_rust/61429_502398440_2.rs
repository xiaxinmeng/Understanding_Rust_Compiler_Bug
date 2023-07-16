asm
  	cmpq	%rdx, %r8
	jbe	.LBB8_7
	movq	(%rcx), %rax
	movb	$1, 88(%rax,%rsi)
	movq	16(%rcx), %r8
	movq	24(%rcx), %rax
	movq	%rdx, 24(%rcx)
	cmpq	%rax, %r8
	jbe	.LBB8_15
	cmpq	%rdx, %r8
	jbe	.LBB8_13
	movq	(%rcx), %rcx
	leaq	(%rax,%rax,2), %rax
	shlq	$5, %rax
	leaq	(%rcx,%rax), %rdi
	addq	$32, %rdi
	addq	%rcx, %rsi
	addq	$32, %rsi
	#APP

	movq	%rsp, (%rdi)
	movq	%r15, 8(%rdi)
	movq	%r14, 16(%rdi)
	movq	%r13, 24(%rdi)
	movq	%r12, 32(%rdi)
	movq	%rbx, 40(%rdi)
	movq	%rbp, 48(%rdi)

	movq	(%rsi), %rsp
	movq	8(%rsi), %r15
	movq	16(%rsi), %r14
	movq	24(%rsi), %r13
	movq	32(%rsi), %r12
	movq	40(%rsi), %rbx
	movq	48(%rsi), %rbp
	retq
