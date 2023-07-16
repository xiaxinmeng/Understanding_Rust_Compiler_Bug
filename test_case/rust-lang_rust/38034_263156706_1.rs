asm
	mov	rax, rdi
	imul	rax, rsi
	jno	.LBB0_2
	xor	rsi, rdi
	shr	rsi, 63
	movabs	rax, 9223372036854775807
	add	rax, rsi
.LBB0_2:
	ret
