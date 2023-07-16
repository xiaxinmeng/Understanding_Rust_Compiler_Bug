asm
_ZN2tt12do_something17h17355b1b96872ed5E:
.Lfunc_begin9:
.seh_proc _ZN2tt12do_something17h17355b1b96872ed5E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	cmp	qword ptr [rcx + 16], 0
	je	.LBB9_5
	mov	rax, qword ptr [rcx]
	mov	r8, qword ptr [rax + 16]
	cmp	r8, 4
	jne	.LBB9_2
	mov	rcx, qword ptr [rax]
	lea	rdx, [rip + anon.8da7dae456594b2e933d3f3e66d20965.4]
	call	memcmp
	test	eax, eax
	sete	al
	jmp	.LBB9_4
.LBB9_2:
	xor	eax, eax
.LBB9_4:
	add	rsp, 40
	ret
.LBB9_5:
	lea	r8, [rip + anon.8da7dae456594b2e933d3f3e66d20965.3]
	xor	ecx, ecx
	xor	edx, edx
	call	_ZN4core9panicking18panic_bounds_check17h9bd8ac36ac287196E
	ud2
.Lfunc_end9: 
