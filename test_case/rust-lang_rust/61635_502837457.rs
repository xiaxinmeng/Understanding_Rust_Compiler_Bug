asm
playground::signum: # @playground::signum
# %bb.0:
	xorl	%eax, %eax
	testl	%edi, %edi
	setg	%al
	sarl	$31, %edi
	addl	%edi, %eax
	retq
                                        # -- End function

playground::signum_branch: # @playground::signum_branch
# %bb.0:
	xorl	%ecx, %ecx
	testl	%edi, %edi
	setne	%cl
	movl	$-1, %eax
	cmovnsl	%ecx, %eax
	retq
                                        # -- End function
