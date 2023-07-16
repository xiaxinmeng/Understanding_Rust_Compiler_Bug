x86asm
playground::index:
	shrl	$4, %esi
	andl	$3, %esi
	shlq	$4, %rsi
	movq	(%rdi,%rsi), %rax
	movq	8(%rdi,%rsi), %rdx
	retq
