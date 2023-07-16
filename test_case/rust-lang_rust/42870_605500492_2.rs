
playground::bar:
	pushq	%rbx
	movq	%rdi, %rbx
	callq	*init@GOTPCREL(%rip)
	movq	%rbx, %rax
	popq	%rbx
	retq
