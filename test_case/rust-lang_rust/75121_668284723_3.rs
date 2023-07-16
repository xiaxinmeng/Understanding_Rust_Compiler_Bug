asm
	.text
	.file	"panic.8h59gtm7-cgu.0"
	.section	.text._ZN5panic6substr17h2e99bd59166da18bE,"ax",@progbits
	.globl	_ZN5panic6substr17h2e99bd59166da18bE
	.p2align	4, 0x90
	.type	_ZN5panic6substr17h2e99bd59166da18bE,@function
_ZN5panic6substr17h2e99bd59166da18bE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rax
	movq	%rcx, %rdi
	subq	%rdx, %rdi
	jb	.LBB0_10
	testq	%rdx, %rdx
	je	.LBB0_2
	cmpq	%rdx, %rsi
	je	.LBB0_2
	jbe	.LBB0_10
	cmpb	$-65, (%rax,%rdx)
	jle	.LBB0_10
.LBB0_2:
	testq	%rcx, %rcx
	je	.LBB0_6
	cmpq	%rcx, %rsi
	je	.LBB0_6
	jbe	.LBB0_10
	cmpb	$-65, (%rax,%rcx)
	jle	.LBB0_10
.LBB0_6:
	addq	%rdx, %rax
	movq	%rdi, %rdx
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB0_10:
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_1(%rip), %r8
	movq	%rax, %rdi
	callq	*_ZN4core3str16slice_error_fail17he51ff928bfc853b6E@GOTPCREL(%rip)
	ud2
.Lfunc_end0:
	.size	_ZN5panic6substr17h2e99bd59166da18bE, .Lfunc_end0-_ZN5panic6substr17h2e99bd59166da18bE
	.cfi_endproc

	.type	.L__unnamed_2,@object
	.section	.rodata..L__unnamed_2,"a",@progbits
.L__unnamed_2:
	.ascii	"src/lib.rs"
	.size	.L__unnamed_2, 10

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	.L__unnamed_2
	.asciz	"\n\000\000\000\000\000\000\000\002\000\000\000\006\000\000"
	.size	.L__unnamed_1, 24

	.section	".note.GNU-stack","",@progbits
