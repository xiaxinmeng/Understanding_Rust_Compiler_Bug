
; In test1
	vmovaps	.LCPI0_0(%rip), %ymm0           # ymm0 = [0,1,2,3]
	callq	test3@PLT
; In test2
	movaps	.LCPI1_0(%rip), %xmm0           # xmm0 = [0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0]
	movaps	.LCPI1_1(%rip), %xmm1           # xmm1 = [2,3]
	callq	test3@PLT
