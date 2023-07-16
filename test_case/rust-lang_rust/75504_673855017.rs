
__vector_9:
*	push	r0
*	push	r1
*	in	r0, 63
*	push	r0
*	clr	r0
*	push	r24
	push	r25
	push	r28
	push	r29
	in	r28, 61
	in	r29, 62
	sbiw	r28, 1
	in	r0, 63
	cli
	out	62, r29
	out	63, r0
	out	61, r28
*	lds	r24, _ZN16min_repro_broken4FAIL17hb143df4aedca2a54E.0.0
*	cpi	r24, 0
*	breq	LBB0_2
	pop	r29
	pop	r28
	pop	r25
*	pop	r24
*	pop	r0
*	out	63, r0
	adiw	r28, 1
	in	r0, 63
	cli
	out	62, r29
	out	63, r0
	out	61, r28
*	pop	r1
*	pop	r0
*	reti
