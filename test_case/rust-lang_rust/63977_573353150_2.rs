asm
	.p2align	4, 0x90
__ZN1d4main17h3e6e8052bbc66057E:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$112, %rsp
	movq	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h37acbe90c85d9165E@GOTPCREL(%rip), %rsi
	## InlineAsm Start


	xorq	%rax, %rax
.process_loop:
	nop
	addq	$1, %rax
	cmpq	$500, %rax
	jne	.process_loop


	## InlineAsm End
