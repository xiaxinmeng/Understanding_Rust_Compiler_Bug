nasm
.LBB0_5:
	mov	edx, dword ptr [rbx + rcx]
	rol	edx, 8
	bswap	edx
	mov	dword ptr [rax + rcx], edx
	add	rcx, 4
	cmp	rdi, rcx
	jne	.LBB0_5
