
playground::iter: # @playground::iter
# %bb.0:
	sub	rsp, 16

.LBB0_1:                                # =>This Inner Loop Header: Depth=1
	mov	byte ptr [rsp + 8], 0
	movzx	eax, byte ptr [rsp]
	movzx	eax, byte ptr [rsp + 1]
	movzx	eax, byte ptr [rsp + 2]
	movzx	eax, byte ptr [rsp + 3]
	movzx	eax, byte ptr [rsp + 4]
	movzx	eax, byte ptr [rsp + 5]
	movzx	eax, byte ptr [rsp + 6]
	movzx	eax, byte ptr [rsp + 7]
	movzx	eax, byte ptr [rsp + 9]
	movzx	eax, byte ptr [rsp + 10]
	movzx	eax, byte ptr [rsp + 11]
	movzx	eax, byte ptr [rsp + 8]
	jmp	.LBB0_1
                                        # -- End function

playground::ready_bench: # @playground::ready_bench
# %bb.0:
	push	rax
	call	playground::iter
	ud2
                                        # -- End function
