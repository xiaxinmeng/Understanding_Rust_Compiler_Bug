asm
	sub	rsp, 24
	mov	rax, rcx
	mov	qword ptr [rcx], 4
	xorps	xmm0, xmm0
	movups	xmmword ptr [rcx + 8], xmm0
	mov	rcx, qword ptr [rcx + 16]
	mov	qword ptr [rsp + 16], rcx
	mov	rcx, qword ptr [rax]
	mov	qword ptr [rsp], rcx
	mov	rcx, qword ptr [rax + 8]
	mov	qword ptr [rsp + 8], rcx
	mov	rcx, qword ptr [rdx + 16]
	mov	qword ptr [rax + 16], rcx
	movups	xmm0, xmmword ptr [rdx]
	movups	xmmword ptr [rax], xmm0
	mov	rcx, qword ptr [rsp + 16]
	mov	qword ptr [rdx + 16], rcx
	mov	rcx, qword ptr [rsp]
	mov	qword ptr [rdx], rcx
	mov	rcx, qword ptr [rsp + 8]
	mov	qword ptr [rdx + 8], rcx
	add	rsp, 24
	ret
