asm
playground::do_ants: # @playground::do_ants
# %bb.0:
	subq	$216, %rsp
	movq	$0, (%rsp)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 198(%rsp)
	movups	%xmm0, 184(%rsp)
	movq	%rsp, %rdi
	callq	*ext@GOTPCREL(%rip)
	addq	$216, %rsp
	retq
