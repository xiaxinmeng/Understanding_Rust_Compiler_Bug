
foo:
	cmpq	%rsi, %rdi  ; compare array pointers
	je	.LBB0_1     ; same array? return `true` early
	movdqu	(%rdi), %xmm0  ; move one array into XMM0 (16 Byte register)
	movdqu	(%rsi), %xmm1  ; move the other array into XMM1
	pcmpeqb	%xmm0, %xmm1 ; byte-wise compare arrays, set byte in XMM1 to `0xFF` when equal, to `0x00` if not
	pmovmskb	%xmm1, %eax  ; move the MSb of all result bytes into the low 16 bits of `eax`
	cmpl	$65535, %eax  ; compare packed mask with 65535 (all 16 bits set)
	sete	%al  ; set return value to 1 if all 16 bits are set (equal), to 0 if not
	retq

.LBB0_1:
	movb	$1, %al  ; same array, return `true` early
	retq
