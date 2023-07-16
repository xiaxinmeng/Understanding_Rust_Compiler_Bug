asm
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movl	%edi, (%rsp)
	movl	%esi, 4(%rsp)
	cmpl	%esi, %edi
	jne	.LBB11_1
	movb	$1, %al
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
