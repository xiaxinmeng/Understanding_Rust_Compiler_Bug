asm
bad:
	xorps	%xmm1, %xmm1
	cmpless	%xmm0, %xmm1
	andps	%xmm1, %xmm0
	minss	.LCPI0_0(%rip), %xmm0
	retq

good:
	xorps	%xmm1, %xmm1
	cmpnless	%xmm0, %xmm1
	minss	.LCPI1_0(%rip), %xmm0
	andnps	%xmm0, %xmm1
	movaps	%xmm1, %xmm0
	retq
