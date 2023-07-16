asm
	.section	.text.sum_them,"ax",@progbits
	.globl	sum_them
	.p2align	4, 0x90
	.type	sum_them,@function
sum_them:
	.cfi_startproc
	movl	(%rdi), %eax
	movss	4(%rdi), %xmm0
	movl	(%rsi), %ecx
	movss	4(%rsi), %xmm1
	leaq	.LJTI5_0(%rip), %rsi
	movslq	(%rsi,%rax,4), %rax
	addq	%rsi, %rax
	jmpq	*%rax
.LBB5_1:
	xorl	%eax, %eax
	testl	%ecx, %ecx
	je	.LBB5_8
	retq
.LBB5_3:
	movl	$2, %eax
	cmpl	$2, %ecx
	je	.LBB5_8
.LBB5_9:
	xorl	%eax, %eax
	retq
.LBB5_5:
	movl	$3, %eax
	cmpl	$3, %ecx
	jne	.LBB5_9
.LBB5_8:
	addss	%xmm1, %xmm0
	movl	%eax, (%rdx)
	movss	%xmm0, 4(%rdx)
	movb	$1, %al
	retq
.LBB5_7:
	movl	$1, %eax
	cmpl	$1, %ecx
	jne	.LBB5_9
	jmp	.LBB5_8
.Lfunc_end5:
	.size	sum_them, .Lfunc_end5-sum_them
	.cfi_endproc
	.section	.rodata.sum_them,"a",@progbits
	.p2align	2
.LJTI5_0:
	.long	.LBB5_1-.LJTI5_0
	.long	.LBB5_7-.LJTI5_0
	.long	.LBB5_3-.LJTI5_0
	.long	.LBB5_5-.LJTI5_0
