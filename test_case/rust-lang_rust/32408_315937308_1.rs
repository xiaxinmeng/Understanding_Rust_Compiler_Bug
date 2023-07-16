asm
_ZN10playground3foo17h002f6b2acb4d5b2dE:
.Lfunc_begin2:
	.loc	1 2 0
	.cfi_startproc
	pushq	%rbp
.Ltmp12:
	.cfi_def_cfa_offset 16
.Ltmp13:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
.Ltmp14:
	.cfi_def_cfa_register %rbp
	movq	%rdi, -16(%rbp)
.Ltmp15:
	.loc	1 2 0 prologue_end
	movq	-16(%rbp), %rdi
	movq	%rdi, -8(%rbp)
.Ltmp16:
	.loc	1 3 0
	movq	-8(%rbp), %rdi
	shlq	$1, %rdi
	movq	%rdi, -24(%rbp)
	.loc	1 4 0
	movq	-24(%rbp), %rax
	popq	%rbp
	retq
