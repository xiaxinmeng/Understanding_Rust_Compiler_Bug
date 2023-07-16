asm
_ZN11swap_2_vecs11swap_2_vecs17h9eab7dfc40f308e2E:
	movups	(%rdx), %xmm0
	movups	(%rcx), %xmm1
	movups	%xmm0, (%rcx)
	movups	%xmm1, (%rdx)
	movq	16(%rcx), %r8
	movq	16(%rdx), %rax
	movq	%rax, 16(%rcx)
	movq	%r8, 16(%rdx)
	retq
