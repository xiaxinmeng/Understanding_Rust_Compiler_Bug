x86asm
playground::foo:
	pushq	%rbx
	movq	%rdi, %rbx
	callq	*init@GOTPCREL(%rip)
	movq	%rbx, %rax
	popq	%rbx
	retq

playground::bar:
	pushq	%rbx
	subq	$32, %rsp
	movq	%rdi, %rbx
	movq	%rsp, %rdi
	callq	*init@GOTPCREL(%rip)
	movups	(%rsp), %xmm0
	movups	16(%rsp), %xmm1
	movups	%xmm1, 16(%rbx)
	movups	%xmm0, (%rbx)
	movq	%rbx, %rax
	addq	$32, %rsp
	popq	%rbx
	retq
