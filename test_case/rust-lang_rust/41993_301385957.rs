
_ZN5bench5swith17hb7875e35eb44fb11E:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB0_1
	cmpb	$107, (%rdi)
	sete	%al
	retq
.LBB0_1:
	xorl	%eax, %eax
	retq
