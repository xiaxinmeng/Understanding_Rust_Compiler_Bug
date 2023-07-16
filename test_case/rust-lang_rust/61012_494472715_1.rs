
foo:
	cmpq	%rsi, %rdi
	je	.LBB0_1
	movdqu	(%rdi), %xmm0
	movdqu	(%rsi), %xmm1
	pcmpeqb	%xmm0, %xmm1
	pmovmskb	%xmm1, %eax
	cmpl	$65535, %eax
	sete	%al
	retq

.LBB0_1:
	movb	$1, %al
	retq
