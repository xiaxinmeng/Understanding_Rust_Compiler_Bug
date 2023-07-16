asm
playground::new:
	mov	ecx, 8
	xor	esi, esi
	mov	rax, rdi
	mul	rcx
	mov	r8, rax
	setno	r9b
	jo	.LBB3_7
	mov	edx, 16
	xor	ecx, ecx
	mov	rax, rdi
	mul	rdx
	setno	dl
	jo	.LBB3_7
	mov	sil, r9b
	shl	rsi, 3
	mov	cl, dl
	shl	rcx, 3
	cmp	rcx, rsi
	cmovae	rsi, rcx
	lea	rdx, [r8 + rcx]
	add	rdx, -1
	neg	rcx
	and	rcx, rdx
	sub	rcx, r8
	add	rcx, r8
	jb	.LBB3_7
	add	rax, rcx
	jb	.LBB3_7
	mov	rdx, rsi
	neg	rdx
	cmp	rax, rdx
	ja	.LBB3_7
	test	rsi, rsi
	je	.LBB3_7
	lea	rax, [rsi + 15]
	and	rax, rsi
	jne	.LBB3_7
	mov	rax, rcx
	ret

.LBB3_7:
	push	rbp
	mov	rbp, rsp
	call	core::result::unwrap_failed
	ud2
