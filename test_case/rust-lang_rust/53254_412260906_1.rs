asm
banana:
	subq	$200, %rsp
	movaps	%xmm8, 176(%rsp)
	movaps	%xmm7, 160(%rsp)
	movaps	%xmm6, 144(%rsp)
	movq	$0, 32(%rsp)
	movq	32(%rsp), %xmm6 ; xmm6? Only xmm0-3 are used in x64 calling convention!
	callq	sinf
	movdqa	%xmm0, %xmm8
	movdqa	%xmm6, %xmm0 
	callq	sinf
	punpckldq	%xmm0, %xmm8 
	movdqa	%xmm6, %xmm0
	callq	sinf
	movdqa	%xmm0, %xmm7 
	pshufd	$229, %xmm6, %xmm0 
	callq	sinf
	punpckldq	%xmm0, %xmm7
        ; <snip>
