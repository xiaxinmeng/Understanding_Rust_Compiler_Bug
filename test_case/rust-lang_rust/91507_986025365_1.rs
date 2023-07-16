asm
sum_f32_big_8:
	mov	rax, rdi
	movups	xmm0, xmmword ptr [rsi]
	movups	xmm1, xmmword ptr [rdx]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi], xmm1
	movups	xmm0, xmmword ptr [rsi + 16]
	movups	xmm1, xmmword ptr [rdx + 16]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi + 16], xmm1
	ret

sum_f32_big_16:
	mov	rax, rdi
	movups	xmm0, xmmword ptr [rsi]
	movups	xmm1, xmmword ptr [rdx]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi], xmm1
	movups	xmm0, xmmword ptr [rsi + 16]
	movups	xmm1, xmmword ptr [rdx + 16]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi + 16], xmm1
	movups	xmm0, xmmword ptr [rsi + 32]
	movups	xmm1, xmmword ptr [rdx + 32]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi + 32], xmm1
	movups	xmm0, xmmword ptr [rsi + 48]
	movups	xmm1, xmmword ptr [rdx + 48]
	addps	xmm1, xmm0
	movups	xmmword ptr [rdi + 48], xmm1
	ret
