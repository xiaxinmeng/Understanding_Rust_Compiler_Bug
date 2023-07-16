asm
bad:
	.cfi_startproc
	cmpq	$0, (%rdi)
	setne	%al
	retq
