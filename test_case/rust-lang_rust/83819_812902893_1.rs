asm
_ZN8test_cmp7compare17h5c31fd9ed0a26eefE:
	xorl	%eax, %eax
	xorl	%r8d, %r8d
	cmpl	%edx, %ecx
	setne	%r8b
	movq	$-1, %rcx
	cmovaeq	%r8, %rcx
	movl	$0, %edx
	cmovneq	%rcx, %rdx
	addq	$1, %rdx
	cmpq	$1, %rdx
	ja	.LBB0_2
	movb	$1, %al
.LBB0_2:
	retq
