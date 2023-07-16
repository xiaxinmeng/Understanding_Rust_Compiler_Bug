asm
make_bar_bad:
	movl	(%rdi), %eax
	retq

make_bar_bad_broken_inline:
	movups	(%rsi), %xmm0
	movups	%xmm0, (%rdi)
	movq	%rdi, %rax
	retq

make_bar_good:
	movups	(%rsi), %xmm0
	movups	%xmm0, (%rdi)
	movq	%rdi, %rax
	retq
