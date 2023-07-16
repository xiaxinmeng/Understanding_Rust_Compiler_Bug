
playground::new_optimized:
	movl	$16, %edx
	xorl	%ecx, %ecx
	movq	%rdi, %rax
	mulq	%rdx
	setno	%cl
	shlq	$3, %rcx
	leaq	(%rcx,%rdi,8), %rax
	addq	$-1, %rax
	negq	%rcx
	andq	%rax, %rcx
	movq	%rcx, %rax
	retq

playground::original:
	leaq	(,%rdi,8), %rax
	retq
