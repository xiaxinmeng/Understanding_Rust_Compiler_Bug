
playground::iter:
	sub	rsp, 16

.LBB0_1:
	mov	dword ptr [rsp], 0
	mov	eax, dword ptr [rsp + 4]
	mov	eax, dword ptr [rsp + 8]
	mov	eax, dword ptr [rsp]
	jmp	.LBB0_1

playground::ready_bench:
	push	rax
	call	playground::iter
	ud2
