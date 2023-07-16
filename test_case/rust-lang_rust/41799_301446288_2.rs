asm
	mov	rbx, rsi
	shr	rbx, 39
	call	__floatuntisf@PLT
	cmp	rbx, 33554430       ; 0x1fffffe
	jbe	.LBB0_2
	movss	xmm0, dword ptr [rip + .LCPI0_0]
.LBB0_2:
	ret

.LCPI0_0:
	.long	2139095040     ; 0x7f800000
