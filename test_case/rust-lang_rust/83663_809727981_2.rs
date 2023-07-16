asm
_ZN59_$LT$eq_test..Blueprint$u20$as$u20$core..cmp..PartialEq$GT$2eq17he4e85086691c6c20E:
	vmovdqu	(%rcx), %xmm0
	movl	16(%rcx), %eax
	vpcmpeqd	(%rdx), %xmm0, %xmm0
	cmpl	16(%rdx), %eax
	vmovmskps	%xmm0, %eax
	sete	%cl
	cmpb	$15, %al
	sete	%al
	andb	%cl, %al
	retq
