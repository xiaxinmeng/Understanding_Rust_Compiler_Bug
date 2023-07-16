asm
atomic_max_usize:
	.fnstart
	.save	{r11, lr}
	push	{r11, lr}
	.pad	#8
	sub	sp, sp, #8
	ldr	r0, [r0]
	ldr	r1, [r1]
	str	r0, [sp, #4]
	add	r0, sp, #4
	bl	__sync_fetch_and_umax_4
	add	sp, sp, #8
	pop	{r11, pc}
