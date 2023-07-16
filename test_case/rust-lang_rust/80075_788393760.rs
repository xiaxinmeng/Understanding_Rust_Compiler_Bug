asm
	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.globl	__ZN4test3foo17hbf89db3e2fd7b5fcE
	.p2align	4, 0x90
__ZN4test3foo17hbf89db3e2fd7b5fcE:
	.cfi_startproc
	xorl	%eax, %eax
	cmpq	%rdx, %rsi
	jbe	LBB0_3
	cmpq	%rdx, %rcx
	ja	LBB0_3
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movb	(%rdi,%rcx), %al
	popq	%rbp
LBB0_3:
	retq
	.cfi_endproc
