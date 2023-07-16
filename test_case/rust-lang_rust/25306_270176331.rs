x86asm
.LBB0_2:
	cmpb	$10, (%rdi,%rax)
	je	.LBB0_4
	incq	%rax
	cmpq	%rax, %rsi
	jne	.LBB0_2
