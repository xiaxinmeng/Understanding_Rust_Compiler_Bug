asm
__ZN12thread_local20get_the_thread_local17h3ebe4a7ff0ca9672E:
	pushq	%rax
	movq	__ZN12thread_local20get_the_thread_local1X7__getit5__KEY17hcd2875a690ec9ddbE@TLVP(%rip), %rdi
	callq	*(%rdi)
	cmpq	$0, (%rax)
	je	LBB10_1
LBB10_2:
	movq	16(%rax), %rax
	popq	%rcx
	retq
LBB10_1:
	callq	__ZN3std6thread5local4fast12Key$LT$T$GT$19try_initialize_drop17hb0e61de580a0b1eeE
	testq	%rax, %rax
	jne	LBB10_2
	callq	__ZN4core6result13unwrap_failed17h6d9d014b9b2876d6E
