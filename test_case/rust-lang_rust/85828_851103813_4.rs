x86asm
	sub	rsp, 16
	mov	qword ptr [rsp + 8], rsi
	mov	qword ptr [rsp], rdi
	or	rdi, rsi
	sete	al
	add	rsp, 16
	ret
