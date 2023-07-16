asm
.LBB0_1:
	inc	ebp
	mov	rdi, rbx
	call	r14
	cmp	ebp, 200
	jl	.LBB0_1
