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
	jb	.LBB2_6             # EXTRA BRANCH HERE
# %bb.1:
	mov	rbx, qword ptr [rdi]
	mov	qword ptr [rdi + 16], r15
	jne	.LBB2_3      
# %bb.2:
	mov	r13, r15
	jmp	.LBB2_8

.LBB2_3:
	mov	qword ptr [rsp], rdi    # 8-byte Spill
	lea	rbp, [8*r15]
	shl	r13, 3
	mov	r14, qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB2_4:                                # =>This Inner Loop Header: Depth=1
	mov	rdi, qword ptr [rbx + rbp]
	mov	esi, 4
	mov	edx, 4
	call	r14
	add	rbp, 8
	cmp	r13, rbp
	jne	.LBB2_4
# %bb.5:
	mov	rdi, qword ptr [rsp]    # 8-byte Reload
	mov	r13, qword ptr [rdi + 16]

.LBB2_6:
	cmp	r15, r13
	jb	.LBB2_17
# %bb.7:
	mov	rbx, qword ptr [rdi]

.LBB2_8:
	mov	rcx, qword ptr [r12]
	lea	rsi, [rcx + 8*r13]
	sub	r15, r13
	test	r13, r13
	je	.LBB2_16
# %bb.9:
	lea	rdx, [r13 - 1]
	mov	r8d, r13d
	and	r8d, 3
	cmp	rdx, 3
	jae	.LBB2_11
# %bb.10:
	xor	edx, edx
	jmp	.LBB2_13

.LBB2_11:
	sub	r13, r8
	xor	edx, edx

.LBB2_12:                               # =>This Inner Loop Header: Depth=1
	mov	rax, qword ptr [rbx + 8*rdx]
	mov	rbp, qword ptr [rcx + 8*rdx]
	mov	ebp, dword ptr [rbp]
	mov	dword ptr [rax], ebp
	mov	rax, qword ptr [rbx + 8*rdx + 8]
	mov	rbp, qword ptr [rcx + 8*rdx + 8]
	mov	ebp, dword ptr [rbp]
	mov	dword ptr [rax], ebp
	mov	rax, qword ptr [rbx + 8*rdx + 16]
	mov	rbp, qword ptr [rcx + 8*rdx + 16]
	mov	ebp, dword ptr [rbp]
	mov	dword ptr [rax], ebp
	mov	rax, qword ptr [rbx + 8*rdx + 24]
	mov	rbp, qword ptr [rcx + 8*rdx + 24]
	add	rdx, 4
	mov	ebp, dword ptr [rbp]
	mov	dword ptr [rax], ebp
	cmp	r13, rdx
	jne	.LBB2_12

.LBB2_13:
	test	r8, r8
	je	.LBB2_16
# %bb.14:
	lea	rcx, [rcx + 8*rdx]
	lea	rdx, [rbx + 8*rdx]
	xor	eax, eax

.LBB2_15:                               # =>This Inner Loop Header: Depth=1
	mov	rbp, qword ptr [rdx + 8*rax]
	mov	rbx, qword ptr [rcx + 8*rax]
	mov	ebx, dword ptr [rbx]
	mov	dword ptr [rbp], ebx
	add	rax, 1
	cmp	r8, rax
	jne	.LBB2_15

.LBB2_16:
	mov	rdx, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	jmp	alloc::vec::Vec<T>::extend_from_slice # TAILCALL

.LBB2_17:
	lea	rdx, [rip + .L__unnamed_1]
	mov	rdi, r13
	mov	rsi, r15
	call	qword ptr [rip + core::slice::slice_index_len_fail@GOTPCREL]
	ud2
                                        # -- End function
