asm
	push	{r4, r5, r6, r7, r8, r9, r10, r11, lr}
	ldrd	r8, r9, [r0]
	ldrexd	r10, r11, [r0]
	eor	r1, r11, r9
	eor	r7, r10, r8
	orr	r5, r7, r1
	subs	r7, r2, r8
	sbcs	r7, r3, r9
	mov	r1, #0
	mov	r7, #0
	movwlo	r7, #1
	cmp	r7, #0
	mov	r7, r3
	movne	r7, r9
	mov	r6, r2
	movne	r6, r8
	cmp	r5, #0
	bne	.LBB2_4
	dmb	ish
.LBB2_2:
	strexd	r5, r6, r7, [r0]
	cmp	r5, #0
	beq	.LBB2_6
	ldrexd	r10, r11, [r0]
	eor	r5, r10, r8
	eor	r4, r11, r9
	orrs	r5, r5, r4
	beq	.LBB2_2
.LBB2_4:
	clrex
	cmp	r1, #0
	beq	.LBB2_8
.LBB2_5:
	pop	{r4, r5, r6, r7, r8, r9, r10, r11, pc}
.LBB2_6:
	dmb	ish
	mov	r1, #1
	cmp	r1, #0
	beq	.LBB2_8
	b	.LBB2_5
.LBB2_7:
	mov	r10, r6
	mov	r11, r7
	cmp	r1, #0
	bne	.LBB2_5
.LBB2_8:
	ldrexd	r6, r7, [r0]
	mov	r9, r3
	eor	r1, r6, r10
	eor	r5, r7, r11
	orr	r1, r1, r5
	subs	r5, r2, r10
	sbcs	r5, r3, r11
	mov	r5, #0
	movwlo	r5, #1
	cmp	r5, #0
	movne	r9, r11
	mov	r8, r2
	movne	r8, r10
	cmp	r1, #0
	bne	.LBB2_12
	dmb	ish
.LBB2_10:
	strexd	r1, r8, r9, [r0]
	cmp	r1, #0
	beq	.LBB2_13
	ldrexd	r6, r7, [r0]
	eor	r1, r6, r10
	eor	r4, r7, r11
	orrs	r1, r1, r4
	beq	.LBB2_10
.LBB2_12:
	mov	r1, #0
	clrex
	b	.LBB2_7
.LBB2_13:
	dmb	ish
	mov	r1, #1
	b	.LBB2_7
