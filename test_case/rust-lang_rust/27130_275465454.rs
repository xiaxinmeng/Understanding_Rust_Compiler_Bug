assembly
_ZN8rust_out13trim_in_place17ha198dc1ee798e461E:
	.cfi_startproc
	pushq	%rax
.Ltmp0:
	.cfi_def_cfa_offset 16
	movq	8(%rdi), %rax
	jmp	.LBB0_1
	.p2align	4, 0x90
.LBB0_5:
	incq	(%rdi)
	movq	%rax, 8(%rdi)
.LBB0_1:
	decq	%rax
	cmpq	$-1, %rax
	setne	%cl
	je	.LBB0_3
	movq	(%rdi), %rcx
	cmpb	$42, (%rcx)
	sete	%cl
.LBB0_3:
	testb	%cl, %cl
	je	.LBB0_6
	cmpq	$-1, %rax
	jne	.LBB0_5
	movl	$1, %edi
	xorl	%esi, %esi
	callq	_ZN4core5slice22slice_index_order_fail17h9eb379df958d4186E@PLT
.LBB0_6:
	popq	%rax
	retq
.Lfunc_end0:
	.size	_ZN8rust_out13trim_in_place17ha198dc1ee798e461E, .Lfunc_end0-_ZN8rust_out13trim_in_place17ha198dc1ee798e461E
	.cfi_endproc
