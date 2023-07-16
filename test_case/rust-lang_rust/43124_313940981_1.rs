asm
	.section	.text._ZN8collect215create_manually17h733d7e09fad01d19E,"ax",@progbits
	.globl	_ZN8collect215create_manually17h733d7e09fad01d19E
	.p2align	4, 0x90
	.type	_ZN8collect215create_manually17h733d7e09fad01d19E,@function
_ZN8collect215create_manually17h733d7e09fad01d19E:
	.cfi_startproc
	pushq	%rbx
.Lcfi9:
	.cfi_def_cfa_offset 16
	subq	$48, %rsp
.Lcfi10:
	.cfi_def_cfa_offset 64
.Lcfi11:
	.cfi_offset %rbx, -16
	movq	%rdi, %rbx
	leaq	8(%rsp), %rdx
	movl	$131070, %edi
	movl	$2, %esi
	callq	__rust_alloc_zeroed@PLT
	testq	%rax, %rax
	je	.LBB3_4
	movq	%rax, 8(%rsp)
	movaps	.LCPI3_0(%rip), %xmm0
	movups	%xmm0, 16(%rsp)
	leaq	131070(%rax), %rcx
	xorl	%edx, %edx
	.p2align	4, 0x90
.LBB3_2:
	movw	%dx, (%rax,%rdx,2)
	leaq	1(%rdx), %rsi
	movw	%si, 2(%rax,%rdx,2)
	incl	%esi
	movw	%si, 4(%rax,%rdx,2)
	leaq	3(%rdx), %rsi
	movw	%si, 6(%rax,%rdx,2)
	incl	%esi
	movw	%si, 8(%rax,%rdx,2)
	leaq	10(%rax,%rdx,2), %rsi
	addq	$5, %rdx
	cmpq	%rcx, %rsi
	jne	.LBB3_2
	movq	24(%rsp), %rax
	movq	%rax, 16(%rbx)
	movups	8(%rsp), %xmm0
	movups	%xmm0, (%rbx)
	movq	%rbx, %rax
	addq	$48, %rsp
	popq	%rbx
	retq
.LBB3_4:
	movq	8(%rsp), %rax
	movups	16(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
	movq	%rax, 8(%rsp)
	movaps	32(%rsp), %xmm0
	movups	%xmm0, 16(%rsp)
	leaq	8(%rsp), %rdi
	callq	_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h14166e9ef3dd1a36E
.Lfunc_end3:
	.size	_ZN8collect215create_manually17h733d7e09fad01d19E, .Lfunc_end3-_ZN8collect215create_manually17h733d7e09fad01d19E
	.cfi_endproc

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI4_0:
	.quad	65535
	.quad	65535
