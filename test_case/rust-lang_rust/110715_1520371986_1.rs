asm
_ZN3poc4foo217hf17d37ea238339f3E:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	edi, 128
	mov	esi, 1
	call	qword ptr [rip + __rust_alloc@GOTPCREL]
	test	rax, rax
	je	.LBB0_1
	mov	r14, rax
	mov	rdi, rax
	call	rbx
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB0_1:
	mov	edi, 128
	mov	esi, 1
	call	qword ptr [rip + _ZN5alloc5alloc18handle_alloc_error17h9f155cb3ff8eda02E@GOTPCREL]
	ud2
