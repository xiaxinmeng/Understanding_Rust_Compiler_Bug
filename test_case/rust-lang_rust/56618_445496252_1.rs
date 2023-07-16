
	pushq	%rax
	.cfi_def_cfa_offset 16
	movb	$0, 7(%rdi)
	movw	$0, 5(%rdi)
	movl	$0, 1(%rdi)
	movzwl	-4(%rdi), %eax
	movw	%ax, -128(%rsp)
	movb	-2(%rdi), %al
	movb	%al, -126(%rsp)
	movl	1(%rdi), %eax
	movl	%eax, -123(%rsp)
	movzwl	5(%rdi), %eax
	movw	%ax, -119(%rsp)
	movb	7(%rdi), %al
	movb	%al, -117(%rsp)
	movl	8(%rdi), %eax
	movl	%eax, -116(%rsp)
	movb	-124(%rsp), %al
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
