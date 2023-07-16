asm
	.section	.text.__vector_0,"ax",@progbits
	.globl	__vector_0
	.p2align	1
	.type	__vector_0,@function
__vector_0:
	push	r0
	push	r1
	in	r0, 63
	push	r0
	clr	r0
	push	r24
	push	r25
	push	r28
	push	r29
	in	r28, 61
	in	r29, 62
	sbiw	r28, 1
	in	r0, 63
	cli
	out	62, r29
	out	63, r0
	out	61, r28
	ldi	r24, 0
	std	Y+1, r24
	movw	r24, r28
	adiw	r24, 1
	call	_ZN13codegen_bug_27use_var17h8e808e1926aec402E
	pop	r29
	pop	r28
	pop	r25
	pop	r24
	pop	r0
	out	63, r0
	adiw	r28, 1
	in	r0, 63
	cli
	out	62, r29
	out	63, r0
	out	61, r28
	pop	r1
	pop	r0
	reti
.Lfunc_end0:
	.size	__vector_0, .Lfunc_end0-__vector_0
