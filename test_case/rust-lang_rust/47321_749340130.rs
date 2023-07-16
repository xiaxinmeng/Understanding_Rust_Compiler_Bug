asm
	.section	.text._ZN6cursor4main17h6a4e86eb354fd049E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN6cursor4main17h6a4e86eb354fd049E,@function
_ZN6cursor4main17h6a4e86eb354fd049E:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$80, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movb	$1, 15(%rsp)
	leaq	15(%rsp), %r14
	movq	%r14, 16(%rsp)
	movq	_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hd2673ece5df91901E@GOTPCREL(%rip), %r15
	movq	%r15, 24(%rsp)
	leaq	.L__unnamed_2(%rip), %r13
	movq	%r13, 32(%rsp)
	movq	$2, 40(%rsp)
	movq	$0, 48(%rsp)
	leaq	16(%rsp), %rbx
	movq	%rbx, 64(%rsp)
	movq	$1, 72(%rsp)
	movq	_ZN3std2io5stdio6_print17h0d31d4b9faa6e1ecE@GOTPCREL(%rip), %r12
	leaq	32(%rsp), %rdi
	callq	*%r12
	movb	$2, 15(%rsp)
	movq	%r14, 16(%rsp)
	movq	%r15, 24(%rsp)
	movq	%r13, 32(%rsp)
	movq	$2, 40(%rsp)
	movq	$0, 48(%rsp)
	movq	%rbx, 64(%rsp)
	movq	$1, 72(%rsp)
	leaq	32(%rsp), %rdi
	callq	*%r12
	movb	$3, 15(%rsp)
	movq	%r14, 16(%rsp)
	movq	%r15, 24(%rsp)
	movq	%r13, 32(%rsp)
	movq	$2, 40(%rsp)
	movq	$0, 48(%rsp)
	movq	%rbx, 64(%rsp)
	movq	$1, 72(%rsp)
	leaq	32(%rsp), %rdi
	callq	*%r12
	movb	$4, 15(%rsp)
	movq	%r14, 16(%rsp)
	movq	%r15, 24(%rsp)
	movq	%r13, 32(%rsp)
	movq	$2, 40(%rsp)
	movq	$0, 48(%rsp)
	movq	%rbx, 64(%rsp)
	movq	$1, 72(%rsp)
	leaq	32(%rsp), %rdi
	callq	*%r12
	addq	$80, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN6cursor4main17h6a4e86eb354fd049E, .Lfunc_end5-_ZN6cursor4main17h6a4e86eb354fd049E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_ZN6cursor4main17h6a4e86eb354fd049E(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.L__unnamed_1(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17h142b9cc66267fea1E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	main, .Lfunc_end6-main
	.cfi_endproc
