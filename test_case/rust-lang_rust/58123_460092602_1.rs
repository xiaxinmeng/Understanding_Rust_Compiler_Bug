asm
	.text
	.file	"peekmut.71651wce-cgu.0"
	.section	.text._ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E,@function
_ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpb	$0, 8(%rdi)
	je	.LBB0_12
	movq	(%rdi), %rax
	movq	16(%rax), %r9
	testq	%r9, %r9
	je	.LBB0_13
	movq	(%rax), %rax
	movl	(%rax), %r8d
	cmpq	$1, %r9
	jne	.LBB0_4
	xorl	%esi, %esi
	jmp	.LBB0_11
.LBB0_4:
	movl	$1, %edi
	xorl	%ecx, %ecx
	.p2align	4, 0x90
.LBB0_5:
	leaq	1(%rdi), %rsi
	cmpq	%r9, %rsi
	jae	.LBB0_8
	movl	(%rax,%rdi,4), %edx
	cmpl	4(%rax,%rdi,4), %edx
	ja	.LBB0_8
	movq	%rsi, %rdi
.LBB0_8:
	movq	%rdi, %rsi
	movl	(%rax,%rdi,4), %edi
	cmpl	%edi, %r8d
	jae	.LBB0_9
	movl	%edi, (%rax,%rcx,4)
	leaq	(%rsi,%rsi), %rdi
	addq	$1, %rdi
	movq	%rsi, %rcx
	cmpq	%r9, %rdi
	jb	.LBB0_5
	jmp	.LBB0_11
.LBB0_9:
	movq	%rcx, %rsi
.LBB0_11:
	movl	%r8d, (%rax,%rsi,4)
.LBB0_12:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.LBB0_13:
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_1(%rip), %rdi
	xorl	%esi, %esi
	xorl	%edx, %edx
	callq	*_ZN4core9panicking18panic_bounds_check17h59684c930baf7d8bE@GOTPCREL(%rip)
	ud2
.Lfunc_end0:
	.size	_ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E, .Lfunc_end0-_ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E
	.cfi_endproc

	.section	.text.is_opt,"ax",@progbits
	.globl	is_opt
	.p2align	4, 0x90
	.type	is_opt,@function
is_opt:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	cmpq	$0, 16(%rdi)
	je	.LBB1_18
	movq	%rdi, 8(%rsp)
	movb	$1, 16(%rsp)
	cmpq	$0, 16(%rdi)
	je	.LBB1_2
	movq	(%rdi), %rax
	movl	$1, (%rax)
	movq	16(%rdi), %r9
	testq	%r9, %r9
	je	.LBB1_6
	movq	(%rdi), %rax
	movl	(%rax), %r8d
	cmpq	$1, %r9
	jne	.LBB1_9
	xorl	%esi, %esi
	jmp	.LBB1_16
.LBB1_18:
	xorl	%ecx, %ecx
	movl	%ecx, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB1_9:
	.cfi_def_cfa_offset 32
	movl	$1, %edi
	xorl	%ecx, %ecx
	.p2align	4, 0x90
.LBB1_10:
	leaq	1(%rdi), %rsi
	cmpq	%r9, %rsi
	jae	.LBB1_13
	movl	(%rax,%rdi,4), %edx
	cmpl	4(%rax,%rdi,4), %edx
	ja	.LBB1_13
	movq	%rsi, %rdi
.LBB1_13:
	movq	%rdi, %rsi
	movl	(%rax,%rdi,4), %edi
	cmpl	%edi, %r8d
	jae	.LBB1_14
	movl	%edi, (%rax,%rcx,4)
	leaq	(%rsi,%rsi), %rdi
	addq	$1, %rdi
	movq	%rsi, %rcx
	cmpq	%r9, %rdi
	jb	.LBB1_10
	jmp	.LBB1_16
.LBB1_14:
	movq	%rcx, %rsi
.LBB1_16:
	movl	%r8d, (%rax,%rsi,4)
	movb	$1, %cl
	movl	%ecx, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB1_2:
	.cfi_def_cfa_offset 32
.Ltmp3:
	leaq	.L__unnamed_2(%rip), %rdi
	xorl	%esi, %esi
	xorl	%edx, %edx
	callq	*_ZN4core9panicking18panic_bounds_check17h59684c930baf7d8bE@GOTPCREL(%rip)
.Ltmp4:
	jmp	.LBB1_3
.LBB1_6:
.Ltmp0:
	leaq	.L__unnamed_1(%rip), %rdi
	xorl	%esi, %esi
	xorl	%edx, %edx
	callq	*_ZN4core9panicking18panic_bounds_check17h59684c930baf7d8bE@GOTPCREL(%rip)
.Ltmp1:
.LBB1_3:
	ud2
.LBB1_4:
.Ltmp2:
	ud2
	ud2
.LBB1_19:
.Ltmp5:
	leaq	8(%rsp), %rdi
	callq	_ZN4core3ptr18real_drop_in_place17hb6242b27b5771784E
	ud2
	ud2
.Lfunc_end1:
	.size	is_opt, .Lfunc_end1-is_opt
	.cfi_endproc
	.section	.gcc_except_table,"a",@progbits
	.p2align	2
GCC_except_table1:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end1-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2

	.type	str.0,@object
	.section	.rodata.str.0,"a",@progbits
	.p2align	4
str.0:
	.ascii	"/rustc/8a57831a4b7dfa960110599748f3b7382ae28237/src/libcore/slice/mod.rs"
	.size	str.0, 72

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	str.0
	.quad	72
	.long	2541
	.long	14
	.size	.L__unnamed_2, 24

	.type	str.1,@object
	.section	.rodata.str.1,"a",@progbits
	.p2align	4
str.1:
	.ascii	"/rustc/8a57831a4b7dfa960110599748f3b7382ae28237/src/liballoc/collections/binary_heap.rs"
	.size	str.1, 87

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	str.1
	.quad	87
	.long	868
	.long	30
	.size	.L__unnamed_1, 24

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality

	.section	".note.GNU-stack","",@progbits
