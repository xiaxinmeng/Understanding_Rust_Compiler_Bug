asm
playground::foo:
	movq	8(%rdi), %rsi
	testq	%rsi, %rsi
	je	.LBB0_2
	pushq	%rax
	movq	(%rdi), %rdi
	shlq	$2, %rsi
	movl	$4, %edx
	callq	__rust_dealloc@PLT
	addq	$8, %rsp

.LBB0_2:
	xorl	%eax, %eax
	retq
