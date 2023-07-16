asm
_ZN4test3bar17h0bd71e8ae5322970E:
	push	r15
	push	r14
	push	rbx
	mov	ebx, 42
	call	qword ptr [rip + foo@GOTPCREL]
.LBB1_4:
	mov	eax, ebx
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB1_1:
	mov	rdi, rax
	call	qword ptr [rip + _ZN3std9panicking3try7cleanup17hee23cc19e10d1537E@GOTPCREL]
	mov	r14, rax
	mov	r15, rdx
	mov	rdi, rax
	call	qword ptr [rdx]
	mov	rsi, qword ptr [r15 + 8]
	mov	ebx, 13
	test	rsi, rsi
	je	.LBB1_4
	mov	rdx, qword ptr [r15 + 16]
	mov	rdi, r14
	call	qword ptr [rip + __rust_dealloc@GOTPCREL]
	jmp	.LBB1_4
.LBB1_5:
	mov	rbx, rax
	mov	rdi, r14
	mov	rsi, r15
	call	_ZN5alloc5alloc8box_free17h849e57dccc1d906aE
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
	ud2
