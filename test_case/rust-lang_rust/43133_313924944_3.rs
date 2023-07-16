asm
__1::reverse2::hbb951fc559ba0b9c:
	movb	4(%esp), %al
	movl	%eax, %ecx
	negb	%cl
	sbbb	%cl, %cl
	cmpb	$-1, %al
	movb	$1, %al
	je	LBB0_2
	movl	%ecx, %eax
LBB0_2:
	retl

__1::reverse3::hf325c37bc61a71c6:
	xorl	%eax, %eax
	subb	4(%esp), %al
	retl
