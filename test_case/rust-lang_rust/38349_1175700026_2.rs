asm

id_result:                              # @id_result
# %bb.0:
	movl	%esi, %edx
	xorl	%eax, %eax
	testl	%edi, %edi
	setne	%al
	retq
                                        # -- End function
