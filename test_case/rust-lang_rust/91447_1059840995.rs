asm
.LBB1_4:                                # =>This Inner Loop Header: Depth=1
	movups	xmm1, xmmword ptr [rsi + rdx]
	addps	xmm0, xmm1
	add	rdx, 16
	cmp	rcx, rdx
	jne	.LBB1_4
