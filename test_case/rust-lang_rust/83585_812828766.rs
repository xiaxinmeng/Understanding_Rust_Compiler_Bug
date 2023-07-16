asm
playground::foo1a: # @playground::foo1a
# %bb.0:
	cmpb	%sil, %dil
	jne	.LBB0_6
# %bb.1:
	movq	%rsi, %rcx
	shrq	$8, %rcx
	movq	%rdi, %rdx
	shrq	$8, %rdx
	xorl	%eax, %eax
	cmpb	%cl, %dl
	jne	.LBB0_7
# %bb.2:
	movq	%rsi, %rcx
	shrq	$16, %rcx
	movq	%rdi, %rdx
	shrq	$16, %rdx
	cmpb	%cl, %dl
	jne	.LBB0_7
# %bb.3:
	movq	%rdi, %rcx
	shrq	$24, %rcx
	movq	%rsi, %rdx
	shrq	$24, %rdx
	xorl	%eax, %eax
	cmpb	%dl, %cl
	jne	.LBB0_7
# %bb.4:
	movq	%rdi, %rcx
	shrq	$32, %rcx
	movq	%rsi, %rdx
	shrq	$32, %rdx
	cmpb	%dl, %cl
	jne	.LBB0_7
# %bb.5:
	movq	%rdi, %r8
	shrq	$40, %r8
	movq	%rdi, %rcx
	shrq	$48, %rcx
	shrq	$56, %rdi
	movq	%rsi, %rdx
	shrq	$40, %rdx
	movq	%rsi, %rax
	shrq	$48, %rax
	shrq	$56, %rsi
	xorb	%r8b, %dl
	xorb	%cl, %al
	orb	%dl, %al
	xorb	%dil, %sil
	orb	%al, %sil
	sete	%al
                                        # kill: def $al killed $al killed $eax
	retq

.LBB0_6:
	xorl	%eax, %eax

.LBB0_7:
                                        # kill: def $al killed $al killed $eax
	retq
