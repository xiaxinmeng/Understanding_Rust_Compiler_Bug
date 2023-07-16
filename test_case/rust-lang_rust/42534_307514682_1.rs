
	.text
	.file	"foo.cgu-0.rs"
	.section	.text._ZN3foo3foo17h6610c3c3ce7ddf2aE,"ax",@progbits
	.globl	_ZN3foo3foo17h6610c3c3ce7ddf2aE
	.p2align	4, 0x90
	.type	_ZN3foo3foo17h6610c3c3ce7ddf2aE,@function
_ZN3foo3foo17h6610c3c3ce7ddf2aE:
	.cfi_startproc
	pushq	%rbp
.Lcfi0:
	.cfi_def_cfa_offset 16
	pushq	%r15
.Lcfi1:
	.cfi_def_cfa_offset 24
	pushq	%r14
.Lcfi2:
	.cfi_def_cfa_offset 32
	pushq	%rbx
.Lcfi3:
	.cfi_def_cfa_offset 40
	pushq	%rax
.Lcfi4:
	.cfi_def_cfa_offset 48
.Lcfi5:
	.cfi_offset %rbx, -40
.Lcfi6:
	.cfi_offset %r14, -32
.Lcfi7:
	.cfi_offset %r15, -24
.Lcfi8:
	.cfi_offset %rbp, -16
	movq	%rdi, %rbx
	movq	%rbx, %r14
	shrq	$32, %r14
	xorl	%r15d, %r15d
	xorl	%eax, %eax
	xorl	%ebp, %ebp
	testb	$1, %al
	jne	.LBB0_3
	jmp	.LBB0_2
	.p2align	4, 0x90
.LBB0_9:
	shrq	$32, %rcx
	addl	%ebp, %ecx
	movb	$1, %al
	movl	%ecx, %ebp
	testb	$1, %al
	je	.LBB0_2
.LBB0_3:
	movl	$2, %edi
	callq	_ZN58_$LT$i32$u20$as$u20$core..num..cast..Cast$LT$usize$GT$$GT$4cast17hc851b0f2c71d4b6fE@PLT
	testl	%eax, %eax
	je	.LBB0_5
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	jmp	.LBB0_6
	.p2align	4, 0x90
.LBB0_2:
	movq	%rbx, %rax
	shlq	$32, %rax
	xorl	%ecx, %ecx
	cmpl	%r14d, %ebx
	setl	%cl
	cmovgeq	%r15, %rax
	addl	%ecx, %ebx
	orq	%rax, %rcx
	testl	%ecx, %ecx
	jne	.LBB0_9
	jmp	.LBB0_8
	.p2align	4, 0x90
.LBB0_5:
	shrq	$32, %rax
	addl	%ebx, %eax
	seto	%sil
	movq	%rax, %rdx
	shlq	$32, %rdx
	xorb	$1, %sil
	cmpl	%eax, %r14d
	setg	%cl
	incl	%eax
	andb	%sil, %cl
	cmovel	%ebx, %eax
	movzbl	%cl, %ecx
	cmoveq	%r15, %rdx
	movl	%eax, %ebx
.LBB0_6:
	orq	%rdx, %rcx
	testl	%ecx, %ecx
	jne	.LBB0_9
.LBB0_8:
	movl	%ebp, %eax
	addq	$8, %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
.Lfunc_end0:
	.size	_ZN3foo3foo17h6610c3c3ce7ddf2aE, .Lfunc_end0-_ZN3foo3foo17h6610c3c3ce7ddf2aE
	.cfi_endproc
