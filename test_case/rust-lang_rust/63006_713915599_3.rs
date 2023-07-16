asm
playground::clippy_suggestion:
	movq	%rdi, %rax
	xorps	%xmm0, %xmm0
	movups	%xmm0, 112(%rdi)
	movups	%xmm0, 100(%rdi)
	movups	%xmm0, 84(%rdi)
	movups	%xmm0, 68(%rdi)
	movups	%xmm0, 52(%rdi)
	movups	%xmm0, 36(%rdi)
	movups	%xmm0, 20(%rdi)
	movups	%xmm0, 4(%rdi)
	movl	$-306674912, (%rdi)
	movq	%rdi, %r8
	subq	$-128, %r8
	movl	$1, %edi
	xorl	%ecx, %ecx
	movq	%rax, %rsi
	xorl	%edx, %edx
	testb	$1, %dl
	je	.LBB1_2

.LBB1_4:
	cmpq	%rsi, %r8
	je	.LBB1_6
	testq	%rsi, %rsi
	jne	.LBB1_7
	jmp	.LBB1_6

.LBB1_2:
	movq	%r8, %rdx
	subq	%rsi, %rdx
	shrq	$2, %rdx
	cmpq	%rdi, %rdx
	jbe	.LBB1_6
	leaq	(%rsi,%rdi,4), %rsi
	testq	%rsi, %rsi
	je	.LBB1_6

.LBB1_7:
	movl	$1, %edx
	shll	%cl, %edx
	movl	%edx, (%rsi)
	addq	$4, %rsi
	addl	$1, %ecx
	movb	$1, %dl
	xorl	%edi, %edi
	testb	$1, %dl
	je	.LBB1_2
	jmp	.LBB1_4

.LBB1_6:
	retq
