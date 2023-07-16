asm
playground::get:
	orq	%rdx, %rsi
	cmpq	$7, %rsi
	jbe	.LBB0_2
	xorl	%eax, %eax
	retq

.LBB0_2:
	movb	(%rdi,%rdx), %al
	retq
