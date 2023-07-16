
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
