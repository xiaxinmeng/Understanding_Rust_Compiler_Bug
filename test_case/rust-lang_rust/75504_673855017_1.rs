
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
