asm
	.text
	.file	"peekmut.71651wce-cgu.0"
	.section	.text.is_opt,"ax",@progbits
	.globl	is_opt
	.p2align	4, 0x90
	.type	is_opt,@function
is_opt:
	.cfi_startproc
	cmpq	$0, 16(%rdi)
	je	.LBB0_1
	movq	(%rdi), %rax
	movl	$1, (%rax)
	movq	16(%rdi), %r9
	movb	$1, %al
	cmpq	$2, %r9
	jb	.LBB0_11
	movq	(%rdi), %r10
	movl	(%r10), %r8d
	movl	$1, %esi
	xorl	%edx, %edx
	.p2align	4, 0x90
.LBB0_4:
	leaq	1(%rsi), %rdi
	cmpq	%r9, %rdi
	jae	.LBB0_7
	movl	(%r10,%rsi,4), %ecx
	cmpl	4(%r10,%rsi,4), %ecx
	ja	.LBB0_7
	movq	%rdi, %rsi
.LBB0_7:
	movq	%rsi, %rdi
	movl	(%r10,%rsi,4), %esi
	cmpl	%esi, %r8d
	jae	.LBB0_8
	movl	%esi, (%r10,%rdx,4)
	leaq	(%rdi,%rdi), %rsi
	addq	$1, %rsi
	movq	%rdi, %rdx
	cmpq	%r9, %rsi
	jb	.LBB0_4
	jmp	.LBB0_10
.LBB0_1:
	xorl	%eax, %eax
	retq
.LBB0_8:
	movq	%rdx, %rdi
.LBB0_10:
	movl	%r8d, (%r10,%rdi,4)
.LBB0_11:
	retq
.Lfunc_end0:
	.size	is_opt, .Lfunc_end0-is_opt
	.cfi_endproc


	.section	".note.GNU-stack","",@progbits
