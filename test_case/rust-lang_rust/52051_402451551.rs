asm
_ZN4blah9swap_demo17ha1732a9b71393a7eE:
.seh_proc _ZN4blah9swap_demo17ha1732a9b71393a7eE
	sub	rsp, 32
	.seh_stackalloc 32
	.seh_endprologue
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 16], xmm0
	movups	xmmword ptr [rsp + 6], xmm0
	movzx	eax, word ptr [rcx + 4]
	mov	word ptr [rsp + 4], ax
	mov	eax, dword ptr [rcx]
	mov	dword ptr [rsp], eax
	movzx	eax, word ptr [rdx + 4]
	mov	word ptr [rcx + 4], ax
	mov	eax, dword ptr [rdx]
	mov	dword ptr [rcx], eax
	mov	eax, dword ptr [rsp]
	mov	dword ptr [rdx], eax
	movzx	eax, word ptr [rsp + 4]
	mov	word ptr [rdx + 4], ax
	add	rsp, 32
	ret
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4blah9swap_demo17ha1732a9b71393a7eE
	.seh_endproc
