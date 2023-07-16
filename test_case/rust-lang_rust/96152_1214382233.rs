asm
playground::old_style: # @playground::old_style
# %bb.0:
	movq	%rdi, %rax
	orq	$1, %rax
	retq
                                        # -- End function

playground::cheri_compat: # @playground::cheri_compat
# %bb.0:
	movq	%rdi, %rax
	orq	$1, %rax
	retq
                                        # -- End function

playground::fast: # @playground::fast
# %bb.0:
	movq	%rdi, %rax
	orq	$1, %rax
	retq
                                        # -- End function
