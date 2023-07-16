asm
	movl	$5, %eax
	movl	$16, %edx
	vmovaps	-176(%rbp), %xmm0
	vmovaps	-208(%rbp), %xmm1
	## InlineAsm Start
	pcmpestrm	$0, %xmm1, %xmm0
	## lahf stores the CF, PF, AF, ZF, and SF into %ah, but not the OF.
	## We'll place OF at bit 0 of %ax by conditionally moving.
	movw	$0, %ax
	movw	$1, %dx
	cmovow	%dx, %ax
	lahf
	## InlineAsm End
