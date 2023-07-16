asm
playground::foo_add:
	xorl	%eax, %eax
	movl	%edi, %ecx
	addl	%esi, %ecx
	setns	%al
	addl	$2147483647, %eax
	addl	%esi, %edi
	cmovnol	%edi, %eax
	retq

playground::bar_add:
	addl	%esi, %edi
	movl	$2147483647, %eax
	cmovnol	%edi, %eax
	retq
