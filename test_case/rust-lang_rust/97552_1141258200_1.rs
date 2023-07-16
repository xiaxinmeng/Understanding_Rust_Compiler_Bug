asm
_ZN2tt12do_something17h17355b1b96872ed5E:
.Lfunc_begin6:
.seh_proc _ZN2tt12do_something17h17355b1b96872ed5E
	sub	rsp, 40
	.seh_stackalloc 40
	.seh_endprologue
	cmp	qword ptr [rcx + 16], 0
	je	.LBB6_5
	mov	rax, qword ptr [rcx]
	cmp	qword ptr [rax + 16], 4
	jne	.LBB6_2
	mov	rax, qword ptr [rax]
	cmp	dword ptr [rax], 1953719668
	sete	al
	jmp	.LBB6_4
.LBB6_2:
	xor	eax, eax
.LBB6_4:
	add	rsp, 40
	ret
.LBB6_5:
	lea	r8, [rip + anon.8da7dae456594b2e933d3f3e66d20965.3]
	xor	ecx, ecx
	xor	edx, edx
	call	_ZN4core9panicking18panic_bounds_check17h9bd8ac36ac287196E
	ud2
.Lfunc_end6: 
