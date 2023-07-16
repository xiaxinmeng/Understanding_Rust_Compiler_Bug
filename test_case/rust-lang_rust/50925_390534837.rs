asm
playground::main:
	pushq	%rbx
	subq	$64, %rsp
	movq	$8, 8(%rsp)
	pxor	%xmm0, %xmm0
	movdqu	%xmm0, 16(%rsp)
	movl	$24, %edi
	movl	$8, %esi
	callq	__rust_alloc@PLT
	testq	%rax, %rax
	je	.LBB10_5
	movq	%rax, 8(%rsp)
	movq	$3, 16(%rsp)
	xorl	%ecx, %ecx
	testb	%cl, %cl
	jne	.LBB10_3
	leaq	.Lbyte_str.a+4(%rip), %rcx
	movq	%rcx, %xmm0
	leaq	.Lbyte_str.a(%rip), %rcx
	movq	%rcx, %xmm1
	punpcklqdq	%xmm0, %xmm1
	movdqu	%xmm1, (%rax)
	leaq	.Lbyte_str.a+8(%rip), %rcx
	movq	%rcx, 16(%rax)
	movl	$3, %ecx

.LBB10_3:
	movq	%rcx, 24(%rsp)
	movdqu	8(%rsp), %xmm0
	movdqa	%xmm0, 32(%rsp)
	movq	%rcx, 48(%rsp)
	cmpq	$3, 48(%rsp)
	jne	.LBB10_4
	movq	40(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB10_9
	movq	32(%rsp), %rdi
	shlq	$3, %rsi
	movl	$8, %edx
	callq	__rust_dealloc@PLT

.LBB10_9:
	addq	$64, %rsp
	popq	%rbx
	retq

.LBB10_5:
	callq	alloc::alloc::oom@PLT
	jmp	.LBB10_6

.LBB10_4:
	callq	std::panicking::begin_panic

.LBB10_6:
	ud2
	movq	%rax, %rbx
	leaq	32(%rsp), %rdi
	callq	core::ptr::drop_in_place
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2
	movq	%rax, %rbx
	leaq	8(%rsp), %rdi
	callq	core::ptr::drop_in_place
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2
