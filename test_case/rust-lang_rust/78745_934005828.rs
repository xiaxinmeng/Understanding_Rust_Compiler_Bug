asm
playground::case_1:
	movl	%edi, %eax
	andl	$127, %eax
	cmpl	$64, %edi
	cmovael	%edi, %eax
	retq

playground::case_2:
	movl	%edi, %eax
	andl	$63, %eax
	cmpl	$63, %edi
	cmovael	%edi, %eax
	retq

playground::case_3:
	movl	%edi, %eax
	retq

playground::case_4:
	movl	%edi, %eax
	retq
