asm
playground::R::k: # @playground::R::k
# %bb.0:
	movl	%esi, %eax
	retq
                                        # -- End function

playground::R::j: # @playground::R::j
# %bb.0:
	cmpb	$1, (%rdi)
	leaq	.Lanon.7823f3966c6de2987f3ac86f05619038.0(%rip), %rax
	leaq	.Lanon.7823f3966c6de2987f3ac86f05619038.1(%rip), %rcx
	cmoveq	%rax, %rcx
	addq	$1, %rdi
	jmpq	*24(%rcx)               # TAILCALL
                                        # -- End function
