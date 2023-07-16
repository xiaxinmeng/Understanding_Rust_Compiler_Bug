
playground::bla:
	pushq	%rbx
	movq	%rdi, %rbx
	leaq	.Lanon.2db2a92c049e58c94b228b0f736231fc.0(%rip), %rsi
	movl	$3, %edx
	movl	$3, %ecx
	callq	alloc::str::<impl str>::repeat@PLT
	movq	%rbx, %rax
	popq	%rbx
	retq

.Lanon.2db2a92c049e58c94b228b0f736231fc.0:
	.zero	3,97
