asm
playground::new:
	push	rax
	mov	rsi, rdi
	mov	ecx, 8
	mov	rax, rsi
	mul	rcx
	mov	rcx, rax
	jo	.LBB3_3
	mov	edx, 16
	mov	rax, rsi
	mul	rdx
	jo	.LBB3_3
	add	rax, rcx
	jb	.LBB3_3
	mov	rax, rcx
	pop	rcx
	ret

.LBB3_3:
	call	core::result::unwrap_failed
	ud2
