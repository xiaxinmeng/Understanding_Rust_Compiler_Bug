asm
playground::case_1: # @playground::case_1
# %bb.0:
	cmpl	$127, %edi
	movl	$127, %eax
	cmovbl	%edi, %eax
	retq
                                        # -- End function
.set playground::case_2, playground::case_1
