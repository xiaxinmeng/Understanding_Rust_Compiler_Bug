
playground::unwrap_combinators:
	testl	%edi, %edi
	setne	%cl
	cmpl	%esi, %edx
	setle	%al
	andb	%cl, %al
	retq

playground::unwrap_manual:
	testl	%edi, %edi
	setne	%cl
	cmpl	%edx, %esi
	setge	%al
	andb	%cl, %al
	retq
