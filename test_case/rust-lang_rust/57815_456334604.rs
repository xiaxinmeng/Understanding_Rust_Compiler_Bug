asm
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpl	%esi, %edi
	jne	.LBB10_1
	movb	$1, %al
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
