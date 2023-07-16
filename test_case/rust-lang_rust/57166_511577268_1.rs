
core::ptr::real_drop_in_place:
	movq	%rdi, %rax
	movq	(%rdi), %rdi
	testq	%rdi, %rdi
	je	.LBB0_2
	movq	8(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB0_2
	movl	$1, %edx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)

.LBB0_2:
	retq

playground::foo:
	pushq	%rbx
	subq	$32, %rsp
	movups	(%rsi), %xmm0
	movaps	%xmm0, (%rsp)
	movq	16(%rsi), %rcx
	movq	%rcx, 16(%rsp)
	cmpq	$0, (%rsp)
	je	.LBB1_1
	movq	%rdi, %rax
	movq	16(%rsp), %rcx
	movq	%rcx, 16(%rdi)
	movaps	(%rsp), %xmm0
	movups	%xmm0, (%rdi)
	addq	$32, %rsp
	popq	%rbx
	retq

.LBB1_1:
	leaq	.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.2(%rip), %rdi
	callq	*core::panicking::panic@GOTPCREL(%rip)
	ud2
	movq	%rax, %rbx
	movq	%rsp, %rdi
	callq	core::ptr::real_drop_in_place
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2

.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.0:
	.ascii	"called `Option::unwrap()` on a `None` value"

.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.1:
	.ascii	"src/libcore/option.rs"

.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.2:
	.quad	.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.0
	.asciz	"+\000\000\000\000\000\000"
	.quad	.Lanon.9b27569f3e5c884c6d6bffb1a8af3c8b.1
	.asciz	"\025\000\000\000\000\000\000\000[\001\000\000\025\000\000"
