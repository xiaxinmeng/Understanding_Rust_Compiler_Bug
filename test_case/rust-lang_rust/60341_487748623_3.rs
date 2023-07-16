asm
__ZN12thread_local20get_the_thread_local17ha0569ef21dbb73eaE:
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r14
	pushq	%rbx
	subq	$64, %rsp
	movq	%rdi, %rbx
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	cmpb	$0, 25(%rax)
	jne	LBB4_9
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	cmpb	$0, 24(%rax)
	jne	LBB4_3
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	movq	%rax, %r14
	leaq	__ZN3std6thread5local4fast13destroy_value17h2b601cce828bc311E(%rip), %rsi
	movq	%rax, %rdi
	callq	__ZN3std3sys4unix17fast_thread_local13register_dtor17hea5fd2f6f1030afcE
	movb	$1, 24(%r14)
LBB4_3:
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	cmpq	$0, (%rax)
	je	LBB4_4
LBB4_7:
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	leaq	-48(%rbp), %rdi
	movq	%rax, %rsi
	callq	__ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17h5e0882d4896c9b5eE
	movq	-48(%rbp), %rax
	movq	-40(%rbp), %rcx
	movq	%rcx, -64(%rbp)
	movq	-32(%rbp), %rcx
	movq	%rcx, -56(%rbp)
	testq	%rax, %rax
	je	LBB4_9
	movq	%rax, (%rbx)
	movq	-64(%rbp), %rax
	movq	-56(%rbp), %rcx
	movq	%rcx, 16(%rbx)
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	addq	$64, %rsp
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
LBB4_4:
	leaq	-48(%rbp), %rdi
	callq	__ZN12thread_local4init17h71551753bba23e2bE
	movaps	-48(%rbp), %xmm0
	movaps	%xmm0, -80(%rbp)
	movq	-32(%rbp), %rcx
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h5103305ae64a8553E@TLVP(%rip), %rdi
	callq	*(%rdi)
	movq	(%rax), %rdi
	movq	8(%rax), %rsi
	movaps	-80(%rbp), %xmm0
	movups	%xmm0, (%rax)
	movq	%rcx, 16(%rax)
	testq	%rdi, %rdi
	je	LBB4_7
	testq	%rsi, %rsi
	je	LBB4_7
	movl	$1, %edx
	callq	___rust_dealloc
	jmp	LBB4_7
LBB4_9:
	callq	__ZN4core6result13unwrap_failed17ha41e6baa6f090bd5E
