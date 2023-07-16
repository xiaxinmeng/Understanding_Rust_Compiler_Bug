asm
try_op:
	jmp	try_macro

try_macro:
	xorl	%eax, %eax
	testq	%rdi, %rdi
	setne	%al
	movq	%rsi, %rdx
	retq
