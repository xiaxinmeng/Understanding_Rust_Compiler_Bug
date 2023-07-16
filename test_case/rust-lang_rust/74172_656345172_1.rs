asm
# %bb.0:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	r12, rsi
	mov	r15, qword ptr [rsi + 16]
	mov	r13, qword ptr [rdi + 16]
	cmp	r13, r15
	jbe	.LBB3_4    
# %bb.1:
	mov	rbx, qword ptr [rdi]
	mov	qword ptr [rsp], rdi    # 8-byte Spill
	mov	qword ptr [rdi + 16], r15
	lea	rbp, [8*r15]
	shl	r13, 3
	mov	r14, qword ptr [rip + __rust_dealloc@GOTPCREL] 

.LBB3_2:                                # =>This Inner Loop Header: Depth=1
	mov	rdi, qword ptr [rbx + rbp]
	mov	esi, 4
	mov	edx, 4
	call	r14
	add	rbp, 8
	cmp	r13, rbp
	jne	.LBB3_2
# %bb.3:
	mov	rdi, qword ptr [rsp]    # 8-byte Reload
	mov	r13, qword ptr [rdi + 16]

.LBB3_4:
	mov	rdx, r15
	sub	rdx, r13
	jb	.LBB3_14
# %bb.5:
	mov	r9, qword ptr [r12]
	lea	rsi, [r9 + 8*r13]
	test	r13, r13
	je	.LBB3_13
# %bb.6:
	mov	rcx, qword ptr [rdi]
	lea	rax, [r13 - 1]
	mov	r8d, r13d
	and	r8d, 3
	cmp	rax, 3
	jae	.LBB3_8
# %bb.7:
	xor	eax, eax
	jmp	.LBB3_10

.LBB3_8:
	sub	r13, r8
	xor	eax, eax

.LBB3_9:                                # =>This Inner Loop Header: Depth=1
	mov	rbp, qword ptr [rcx + 8*rax]
	mov	rbx, qword ptr [r9 + 8*rax]
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	mov	rbp, qword ptr [rcx + 8*rax + 8]
	mov	rbx, qword ptr [r9 + 8*rax + 8]
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	mov	rbp, qword ptr [rcx + 8*rax + 16]
	mov	rbx, qword ptr [r9 + 8*rax + 16]
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	mov	rbp, qword ptr [rcx + 8*rax + 24]
	mov	rbx, qword ptr [r9 + 8*rax + 24]
	add	rax, 4
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	cmp	r13, rax
	jne	.LBB3_9

.LBB3_10:
	test	r8, r8
	je	.LBB3_13
# %bb.11:
	lea	r9, [r9 + 8*rax]
	lea	rax, [rcx + 8*rax]
	xor	ecx, ecx

.LBB3_12:                               # =>This Inner Loop Header: Depth=1
	mov	rbp, qword ptr [rax + 8*rcx]
	mov	rbx, qword ptr [r9 + 8*rcx]
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	add	rcx, 1
	cmp	r8, rcx
	jne	.LBB3_12

.LBB3_13:
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	jmp	alloc::vec::Vec<T>::extend_from_slice # TAILCALL

.LBB3_14:
	lea	rdx, [rip + .L__unnamed_1]
	mov	rdi, r13
	mov	rsi, r15
	call	qword ptr [rip + core::slice::slice_index_len_fail@GOTPCREL]
	ud2
                                        # -- End function
