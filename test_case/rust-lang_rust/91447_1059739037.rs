asm
playground::sum: # @playground::sum
# %bb.0:
	movss	xmm0, dword ptr [rsi]           # xmm0 = mem[0],zero,zero,zero
	movss	xmm1, dword ptr [rsi + 4]       # xmm1 = mem[0],zero,zero,zero
	addss	xmm0, dword ptr [rdx]
	addss	xmm1, dword ptr [rdx + 4]
	movss	xmm2, dword ptr [rsi + 8]       # xmm2 = mem[0],zero,zero,zero
	addss	xmm2, dword ptr [rdx + 8]
	mov	rax, rdi
	movss	dword ptr [rdi], xmm0
	movss	dword ptr [rdi + 4], xmm1
	movss	dword ptr [rdi + 8], xmm2
	ret
                                        # -- End function
