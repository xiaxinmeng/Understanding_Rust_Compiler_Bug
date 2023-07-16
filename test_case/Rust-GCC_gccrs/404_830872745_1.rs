
	.file	"test.rs"
	.text
	.globl	__f
	.type	__f, @function
__f:
.LFB0:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	addl	$1, %eax
	cmpl	$-2147483648, %eax
	jne	.L2
	movl	$1, %eax
	jmp	.L4
.L2:
	movl	$0, %eax
.L4:
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	__f, .-__f
	.ident	"GCC: (GNU) 11.0.1 20210325 (experimental)"
	.section	.note.GNU-stack,"",@progbits
