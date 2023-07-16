asm
	sub	rsp, 24
	mov	rax, rcx
	mov	rcx, qword ptr [rdx]
	movups	xmm0, xmmword ptr [rdx + 8]
	movaps	xmmword ptr [rsp], xmm0
	mov	qword ptr [rdx], 4
	xorps	xmm0, xmm0
	movups	xmmword ptr [rdx + 8], xmm0
	mov	qword ptr [rax], rcx
	movaps	xmm0, xmmword ptr [rsp]
	movups	xmmword ptr [rax + 8], xmm0
	add	rsp, 24
	ret
