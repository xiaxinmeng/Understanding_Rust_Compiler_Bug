
.LBB1_1:
	incl	%ebp
	cmpl	$200, %ebp
	jge	.LBB1_2
	movq	16(%rbx), %rdi
	movq	24(%rbx), %rax
	leaq	15(%rdi), %rcx
	negq	%rdi
	andq	%rcx, %rdi
	addq	%r12, %rdi
.Ltmp12:
	callq	*%rax
.Ltmp13:
	jmp	.LBB1_1
