

.LBB0_1:                                # =>This Inner Loop Header: Depth=1
	movq	%rbx, %rdi
	callq	*%r14
	addl	$-1, %ebp
	jne	.LBB0_1
