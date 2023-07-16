`asm
# %bb.0:
	leaq	playground::ARR(%rip), %rdi
	movq	%rdi, %rsi
	jmpq	*foo@GOTPCREL(%rip)     # TAILCALL
                                        # -- End function
