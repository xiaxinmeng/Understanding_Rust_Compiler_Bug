console
$ gcc -o - test.c -O3 -fno-asynchronous-unwind-tables -masm=intel -S
[…]
test:
	sub	rsp, 8
	mov	r8d, 3
	mov	r9d, 3
	mov	edx, 2
	push	4
	mov	ecx, 2
	mov	edi, 1
	mov	esi, 1
	push	4
	call	sum_u128
	xor	rax, 10
	xor	rdx, 10
	or	rax, rdx
	sete	al
	add	rsp, 24
	ret
[…]
