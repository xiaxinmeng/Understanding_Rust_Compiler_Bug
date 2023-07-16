asm

	cmpl	$-2147483648, %edi
	jne	.LBB0_2
	movl	$-2147483648, %eax
	cmpl	$-1, %esi
	je	.LBB0_4
.LBB0_2:
	testl	%esi, %esi
	je	.LBB0_5
	movl	%edi, %eax
	cltd
	idivl	%esi
.LBB0_4:
	popq	%rcx
	retq
.LBB0_5:
	leaq	panic_loc.2(%rip), %rdi
	callq	_ZN4core9panicking5panic17hec1812dcc135e139E@PLT

