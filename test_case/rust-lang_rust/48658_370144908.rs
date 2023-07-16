asm
	dmb	ish
.LBB1_1:
	ldrexd	r4, r5, [r0]
	subs	r1, r2, r4
	sbcs	r1, r3, r5
	mov	r1, #0
	movwlt	r1, #1
	cmp	r1, #0
	mov	r1, r3
	moveq	r4, r2
	movne	r1, r5
	mov	r5, r1
	strexd	r1, r4, r5, [r0]
	cmp	r1, #0
	bne	.LBB1_1
	dmb	ish
