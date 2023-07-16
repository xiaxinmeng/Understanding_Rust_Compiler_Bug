
playground::set_to_zero: # @playground::set_to_zero
# %bb.0:
	pushq	%rax
	movq	16(%rdi), %rdx
	testq	%rdx, %rdx
	je	.LBB0_2
# %bb.1:
	movq	(%rdi), %rdi
	shlq	$2, %rdx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)

.LBB0_2:
	popq	%rax
	retq
                                        # -- End function
