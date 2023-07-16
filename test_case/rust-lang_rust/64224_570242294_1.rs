asm
_ZN4test8drop_box17hee2c4bc13b7e934bE:
	push	r15
	push	r14
	push	rbx
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rsi]
	mov	rsi, qword ptr [rbx + 8]
	test	rsi, rsi
	je	.LBB1_4
	mov	rdx, qword ptr [rbx + 16]
	mov	rdi, r14
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + __rust_dealloc@GOTPCREL]
.LBB1_4:
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB1_3:
	mov	r15, rax
	mov	rdi, r14
	mov	rsi, rbx
	call	_ZN5alloc5alloc8box_free17h849e57dccc1d906aE
	mov	rdi, r15
	call	_Unwind_Resume@PLT
	ud2

_ZN4test3bar17h0bd71e8ae5322970E:
	push	rbx
	mov	ebx, 42
	call	qword ptr [rip + foo@GOTPCREL]
.LBB2_2:
	mov	eax, ebx
	pop	rbx
	ret
.LBB2_1:
	mov	rdi, rax
	call	qword ptr [rip + _ZN3std9panicking3try7cleanup17hee23cc19e10d1537E@GOTPCREL]
	mov	rdi, rax
	mov	rsi, rdx
	call	_ZN4test8drop_box17hee2c4bc13b7e934bE
	mov	ebx, 13
	jmp	.LBB2_2
