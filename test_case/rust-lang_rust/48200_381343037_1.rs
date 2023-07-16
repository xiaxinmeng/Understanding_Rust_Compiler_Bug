asm
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZN11issue_482003foo17h7f3d764b4e2d0d3cE
	.p2align	4, 0x90
__ZN11issue_482003foo17h7f3d764b4e2d0d3cE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorq	$1, %rdi
	xorq	$1, %rdx
	addq	%rcx, %rsi
	xorl	%eax, %eax
	orq	%rdi, %rdx
	cmoveq	%rsi, %rax
	popq	%rbp
	retq
	.cfi_endproc

	.globl	__ZN11issue_482003bar17he38cd8ae2cebf23eE
	.p2align	4, 0x90
__ZN11issue_482003bar17he38cd8ae2cebf23eE:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorq	$1, %rdi
	xorq	$1, %rdx
	addq	%rcx, %rsi
	xorl	%eax, %eax
	orq	%rdi, %rdx
	cmoveq	%rsi, %rax
	popq	%rbp
	retq
	.cfi_endproc

	.globl	__ZN11issue_482003baz17h789590496be40da2E
	.p2align	4, 0x90
__ZN11issue_482003baz17h789590496be40da2E:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	xorq	$1, %rdi
	xorq	$1, %rdx
	addq	%rcx, %rsi
	xorl	%eax, %eax
	orq	%rdi, %rdx
	cmoveq	%rsi, %rax
	popq	%rbp
	retq
	.cfi_endproc


.subsections_via_symbols
