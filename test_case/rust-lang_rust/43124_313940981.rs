asm
	.section	.text._ZN8collect219create_with_collect17h68b620e6057f6286E,"ax",@progbits
	.globl	_ZN8collect219create_with_collect17h68b620e6057f6286E
	.p2align	4, 0x90
	.type	_ZN8collect219create_with_collect17h68b620e6057f6286E,@function
_ZN8collect219create_with_collect17h68b620e6057f6286E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r14
.Lcfi4:
	.cfi_def_cfa_offset 16
	pushq	%rbx
.Lcfi5:
	.cfi_def_cfa_offset 24
	subq	$72, %rsp
.Lcfi6:
	.cfi_def_cfa_offset 96
.Lcfi7:
	.cfi_offset %rbx, -24
.Lcfi8:
	.cfi_offset %r14, -16
	movq	%rdi, %r14
	movq	$2, 48(%rsp)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 56(%rsp)
	movaps	.LCPI2_0(%rip), %xmm0
	movaps	%xmm0, 32(%rsp)
.Ltmp0:
	movq	%rsp, %rdi
	leaq	32(%rsp), %rsi
	movl	$65535, %edx
	callq	_ZN5alloc9allocator6Layout6repeat17hfa9f21888d235374E@PLT
.Ltmp1:
	cmpq	$0, (%rsp)
	je	.LBB2_2
	movq	8(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB2_2
	movq	16(%rsp), %rsi
.Ltmp2:
	movq	%rsp, %rdx
	callq	__rust_alloc@PLT
.Ltmp3:
	testq	%rax, %rax
	je	.LBB2_7
	movq	%rax, 48(%rsp)
	movq	$65535, 56(%rsp)
	movw	$1, %bx
	xorl	%esi, %esi
	xorl	%edx, %edx
	.p2align	4, 0x90
.LBB2_11:
	movzwl	%bx, %edi
	xorl	%ecx, %ecx
	cmpl	$65535, %edi
	setne	%cl
	addl	%ebx, %ecx
	cmpl	$65535, %edi
	movw	%si, (%rax,%rdx,2)
	leaq	1(%rdx), %rdx
	movw	%bx, %si
	movw	%cx, %bx
	jne	.LBB2_11
	movq	%rdx, 64(%rsp)
	movq	%rdx, 16(%r14)
	movups	48(%rsp), %xmm0
	movups	%xmm0, (%r14)
	movq	%r14, %rax
	addq	$72, %rsp
	popq	%rbx
	popq	%r14
	retq
.LBB2_2:
.Ltmp4:
	leaq	str.3(%rip), %rsi
	movq	%rsp, %rdi
	movl	$30, %edx
	callq	_ZN5alloc9allocator8AllocErr13invalid_input17hbab45037cac24347E@PLT
.Ltmp5:
	movq	(%rsp), %rax
	movups	8(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
	jmp	.LBB2_8
.LBB2_7:
	movups	8(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
.LBB2_8:
	movq	%rax, (%rsp)
	movaps	32(%rsp), %xmm0
	movups	%xmm0, 8(%rsp)
.Ltmp6:
	movq	%rsp, %rdi
	callq	_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h14166e9ef3dd1a36E
.Ltmp7:
.LBB2_13:
.Ltmp8:
	movq	%rax, %rbx
	leaq	48(%rsp), %rdi
	callq	_ZN4core3ptr13drop_in_place17hdfdbb953a2d7ca4eE
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end2:
	.size	_ZN8collect219create_with_collect17h68b620e6057f6286E, .Lfunc_end2-_ZN8collect219create_with_collect17h68b620e6057f6286E
	.cfi_endproc
	.section	.gcc_except_table,"a",@progbits
	.p2align	2
GCC_except_table2:
.Lexception0:
	.byte	255
	.byte	155
	.asciz	"\234"
	.byte	3
	.byte	26
	.long	.Ltmp0-.Lfunc_begin0
	.long	.Ltmp7-.Ltmp0
	.long	.Ltmp8-.Lfunc_begin0
	.byte	0
	.long	.Ltmp7-.Lfunc_begin0
	.long	.Lfunc_end2-.Ltmp7
	.long	0
	.byte	0
	.p2align	2

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI3_0:
	.quad	65535
	.quad	65535
