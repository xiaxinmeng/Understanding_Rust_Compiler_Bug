asm
compare_two_arrays:
	cmpq	%r9, %rdx
	jne	.LBB0_1
	incq	%rdx
	movl	$16, %r9d
	.p2align	4, 0x90
.LBB0_3:
	decq	%rdx
	sete	%al
	je	.LBB0_6
	vmovdqu	-16(%rcx,%r9), %xmm0
	vpcmpeqd	-16(%r8,%r9), %xmm0, %xmm0
	vmovmskps	%xmm0, %r10d
	cmpb	$15, %r10b
	jne	.LBB0_6
	movl	(%r8,%r9), %r10d
	leaq	20(%r9), %r11
	cmpl	%r10d, (%rcx,%r9)
	movq	%r11, %r9
	je	.LBB0_3
.LBB0_6:
	retq
.LBB0_1:
	xorl	%eax, %eax
	retq
