asm
__1::reverse2::hbb951fc559ba0b9c:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %eax
	negb	%al
	sbbb	%cl, %cl
	cmpb	$-1, %dil
	movb	$1, %al
	je	LBB0_2
	movl	%ecx, %eax
LBB0_2:
	popq	%rbp
	retq

__1::reverse3::hf325c37bc61a71c6:
	pushq	%rbp
	movq	%rsp, %rbp
	negb	%dil
	movl	%edi, %eax
	popq	%rbp
	retq
