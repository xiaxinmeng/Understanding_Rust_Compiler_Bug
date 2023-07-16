asm
.LBB16_14:
	cmpq	%r15, %rdx
	ja	.LBB16_21
	movdqa	(%r11,%rdx), %xmm0
	pmovmskb	%xmm0, %ebx
	addq	$16, %rdx
	testl	%ebx, %ebx
	je	.LBB16_14
	addq	$-16, %rdx
	testl	%ebx, %ebx
	jne	.LBB16_12
	jmp	.LBB16_17
