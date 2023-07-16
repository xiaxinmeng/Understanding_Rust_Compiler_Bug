asm
_1::reverse2::hbb951fc559ba0b9c:
	ands	r1, r0, #255
	mov	r0, r1
	mvnne	r0, #0
	cmp	r1, #255
	moveq	r0, #1
	bx	lr

_1::reverse3::hf325c37bc61a71c6:
	rsb	r0, r0, #0
	bx	lr
