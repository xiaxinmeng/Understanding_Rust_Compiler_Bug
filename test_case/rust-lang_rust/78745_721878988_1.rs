assembly
playground::case_1:
	cmpl	$127, %edi
	movl	$127, %eax
	cmovbl	%edi, %eax
	andl	$127, %eax
	retq

playground::case_2:
	cmpl	$127, %edi
	movl	$127, %eax
	cmovbl	%edi, %eax
	retq
