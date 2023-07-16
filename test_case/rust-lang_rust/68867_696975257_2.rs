asm
	.section	.text.sum_them,"ax",@progbits
	.globl	sum_them
	.p2align	4, 0x90
	.type	sum_them,@function
sum_them:
	.cfi_startproc
	movl	(%rdi), %eax
	cmpl	%eax, (%rsi)
	jne	.LBB5_1
	movss	4(%rdi), %xmm0
	addss	4(%rsi), %xmm0
	movl	%eax, (%rdx)
	movss	%xmm0, 4(%rdx)
	movb	$1, %al
	retq
.LBB5_1:
	xorl	%eax, %eax
	retq
.Lfunc_end5:
	.size	sum_them, .Lfunc_end5-sum_them
	.cfi_endproc
