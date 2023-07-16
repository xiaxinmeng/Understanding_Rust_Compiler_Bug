asm
_ZN12thread_local20get_the_thread_local17h53234738c97e2accE:
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	subq	$48, %rsp
	movq	%rdi, %r14
	leaq	_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@TLSLD(%rip), %rdi
	callq	__tls_get_addr@PLT
	cmpb	$0, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF+25(%rax)
	jne	.LBB4_8
	movq	%rax, %rbx
	cmpb	$0, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF+24(%rbx)
	jne	.LBB4_3
	movq	%rbx, %rax
	movq	%rax, %r15
	leaq	_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF(%rax), %rdi
	leaq	_ZN3std6thread5local4fast13destroy_value17h5fc509333a064006E(%rip), %rsi
	callq	*_ZN3std3sys4unix17fast_thread_local13register_dtor17h7d5b1813798466baE@GOTPCREL(%rip)
	movb	$1, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF+24(%r15)
.LBB4_3:
	movq	%rbx, %rax
	cmpq	$0, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF(%rbx)
	je	.LBB4_4
.LBB4_7:
	leaq	_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF(%rbx), %rsi
	movq	%rsp, %rdi
	callq	*_ZN60_$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$5clone17h5526323efb1899c4E@GOTPCREL(%rip)
	movq	(%rsp), %rax
	movups	8(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
	testq	%rax, %rax
	je	.LBB4_8
	movq	%rax, (%r14)
	movaps	32(%rsp), %xmm0
	movups	%xmm0, 8(%r14)
	movq	%r14, %rax
	addq	$48, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	retq
.LBB4_4:
	movq	%rsp, %rdi
	callq	_ZN12thread_local4init17h8519144e0d22f3f8E
	movaps	(%rsp), %xmm0
	movq	16(%rsp), %rcx
	movq	%rbx, %rax
	movq	_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF(%rbx), %rdi
	movq	_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF+8(%rbx), %rsi
	movaps	%xmm0, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF(%rbx)
	movq	%rcx, _ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hde48327e2980a364E@DTPOFF+16(%rbx)
	testq	%rdi, %rdi
	je	.LBB4_7
	testq	%rsi, %rsi
	je	.LBB4_7
	movl	$1, %edx
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB4_7
.LBB4_8:
	callq	_ZN4core6result13unwrap_failed17h1d58e680aff0dbf0E
	ud2
