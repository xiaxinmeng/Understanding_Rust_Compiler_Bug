asm
playground::new_optimized:
	mov	rcx, rdi
	mov	edx, 8
	mov	rax, rcx
	mul	rdx
	mov	rsi, rax
	mov	edx, 16
	xor	edi, edi
	mov	rax, rcx
	mul	rdx
	setno	dil
	lea	rax, [rsi + 8*rdi]
	add	rax, -1
	shl	rdi, 3
	neg	rdi
	and	rdi, rax
	mov	rax, rdi
	ret
