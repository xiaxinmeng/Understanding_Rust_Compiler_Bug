asm
100001cfd:	movq	%rdi, %r15         ; save main() Result out param
100001d00:	leaq	307925(%rip), %rsi ; load str constant
100001d07:	leaq	-96(%rbp), %rbx    ; create read() out param
100001d0b:	movl	$9, %edx           ; length of str constant
100001d10:	movq	%rbx, %rdi         ; load read() out param
100001d13:	callq	-1544 <__ZN3std2fs4read17h5248aa1330795a98E>
100001d18:	cmpq	$1, -96(%rbp)      ; check Result discriminant
100001d1d:	jne	20 <__ZN6_507594main17hb99f5070155b7525E+0x43>
100001d1f:	movq	8(%rbx), %rax      ; move Error case into out param
100001d23:	movq	16(%rbx), %rcx
100001d27:	movq	%rcx, 8(%r15)
100001d2b:	movq	%rax, (%r15)
100001d2e:	jmp	149 <__ZN6_507594main17hb99f5070155b7525E+0xD8>
100001d33:	movq	24(%rbx), %rax     ; move Vec into local stack variable h
100001d37:	movq	%rax, -32(%rbp)
100001d3b:	movq	8(%rbx), %rax
100001d3f:	movq	16(%rbx), %rcx
100001d43:	movq	%rcx, -40(%rbp)
100001d47:	movq	%rax, -48(%rbp)
100001d4b:	cmpq	$15, -32(%rbp)     ; assert length of vec (be = below/eq)
100001d50:	jbe	128 <__ZN6_507594main17hb99f5070155b7525E+0xE6>
100001d56:	movq	-48(%rbp), %r14    ; get Vec ptr
100001d5a:	movq	(%r14), %rax       ; copy 16 bytes
100001d5d:	movq	8(%r14), %rcx
100001d61:	movq	%rax, -112(%rbp)
100001d65:	movq	%rcx, -104(%rbp)
