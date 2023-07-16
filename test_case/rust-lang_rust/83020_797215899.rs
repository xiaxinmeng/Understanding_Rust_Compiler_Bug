asm
	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.globl	_index
	.p2align	4, 0x90
_index:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	(%rdi), %eax
	popq	%rbp
	retq
	.cfi_endproc

.subsections_via_symbols

