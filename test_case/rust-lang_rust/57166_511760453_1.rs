
playground::foo:
	pushq	%rax
	testq	%rdi, %rdi
	je	.LBB0_1
	movq	%rdi, %rax
	popq	%rcx
	retq

.LBB0_1:
	leaq	.Lanon.c21f24ad05c622049a6df4006ef31e03.2(%rip), %rdi
	callq	*core::panicking::panic@GOTPCREL(%rip)
	ud2

.Lanon.c21f24ad05c622049a6df4006ef31e03.0:
	.ascii	"called `Option::unwrap()` on a `None` value"

.Lanon.c21f24ad05c622049a6df4006ef31e03.1:
	.ascii	"src/libcore/option.rs"

.Lanon.c21f24ad05c622049a6df4006ef31e03.2:
	.quad	.Lanon.c21f24ad05c622049a6df4006ef31e03.0
	.asciz	"+\000\000\000\000\000\000"
	.quad	.Lanon.c21f24ad05c622049a6df4006ef31e03.1
	.asciz	"\025\000\000\000\000\000\000\000[\001\000\000\025\000\000"
