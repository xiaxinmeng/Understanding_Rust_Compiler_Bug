
_ZN4test10second_foo17h387627478d348617E:
	pushq	%r14
	pushq	%rsi
	pushq	%rdi
	pushq	%rbx
	subq	$40, %rsp
	movq	%rcx, %rsi
	movq	%rsi, %rbx
	shrq	$32, %rbx
	xorl	%r14d, %r14d
	xorl	%eax, %eax
	xorl	%edi, %edi
	testb	$1, %al
	jne	.LBB1_3
	jmp	.LBB1_2
	.p2align	4, 0x90
.LBB1_10:
	shrq	$32, %rcx
	addl	%edi, %ecx
	movb	$1, %al
	movl	%ecx, %edi
	testb	$1, %al
	je	.LBB1_2
.LBB1_3:
	movl	$2, %ecx
	callq	_ZN4core3num69_$LT$impl$u20$core..convert..TryFrom$LT$usize$GT$$u20$for$u20$u32$GT$8try_from17h98ee5a7abea3d10bE
	testl	%eax, %eax
	jne	.LBB1_4
	shrq	$32, %rax
	addl	%esi, %eax
	cmpl	%esi, %eax
	jge	.LBB1_6
.LBB1_4:
	movl	%ebx, %esi
	xorl	%ecx, %ecx
	xorl	%edx, %edx
.LBB1_7:
	orq	%rdx, %rcx
	testl	%ecx, %ecx
	jne	.LBB1_10
	jmp	.LBB1_9
	.p2align	4, 0x90
.LBB1_2:
	movq	%rsi, %rax
	shlq	$32, %rax
	xorl	%ecx, %ecx
	cmpl	%ebx, %esi
	setl	%cl
	cmovgeq	%r14, %rax
	addl	%ecx, %esi
	orq	%rax, %rcx
	testl	%ecx, %ecx
	jne	.LBB1_10
	jmp	.LBB1_9
.LBB1_6:
	movq	%rax, %rcx
	shlq	$32, %rcx
	leal	1(%rax), %esi
	xorl	%edx, %edx
	cmpl	%ebx, %eax
	setl	%dl
	cmovgel	%ebx, %esi
	cmovgeq	%r14, %rcx
	jmp	.LBB1_7
.LBB1_9:
	movl	%edi, %eax
	addq	$40, %rsp
	popq	%rbx
	popq	%rdi
	popq	%rsi
	popq	%r14
	retq
