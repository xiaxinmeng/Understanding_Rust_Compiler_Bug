asm
push	{r7, lr}
mov	r2, r1
asrs	r1, r2, #31
asrs	r3, r0, #31
bl	__aeabi_lmul
...
