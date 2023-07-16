asm
identity:
	.cfi_startproc
	xorl	%eax, %eax
	cmpq	$0, (%rsi)
	movq	8(%rsi), %rcx
	setne	%al
	movq	%rax, (%rdi)
	movq	%rcx, 8(%rdi)
	movq	%rdi, %rax
	retq
