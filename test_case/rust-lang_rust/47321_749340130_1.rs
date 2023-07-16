asm
	.section	.text._ZN6cursor4main17h7a95d388df9093a0E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN6cursor4main17h7a95d388df9093a0E,@function
_ZN6cursor4main17h7a95d388df9093a0E:
.Lfunc_begin179:
	.file	41 "/home/w/Code/tests/rust/cursor/src/main.rs"
	.loc	41 11 0 is_stmt 1
	.cfi_startproc
	subq	$568, %rsp
	.cfi_def_cfa_offset 576
.Ltmp713:
	.loc	41 12 24 prologue_end
	movb	$1, 100(%rsp)
	movb	$2, 101(%rsp)
	movb	$3, 102(%rsp)
	movb	$4, 103(%rsp)
	leaq	100(%rsp), %rdi
.Ltmp714:
	.loc	41 13 19
	callq	_ZN3std2io6cursor15Cursor$LT$T$GT$3new17hedd17a1827a7a97aE
	movq	%rdx, 112(%rsp)
	movq	%rax, 104(%rsp)
.Ltmp715:
	.loc	41 16 20
	leaq	200(%rsp), %rdi
	leaq	104(%rsp), %rsi
	callq	_ZN9byteorder2io12ReadBytesExt7read_u817h564428688e309ffcE
	.loc	41 0 20 is_stmt 0
	leaq	.L__unnamed_17(%rip), %rax
	.loc	41 16 20
	leaq	200(%rsp), %rdi
	movq	%rax, %rsi
	callq	_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h34cf537a266a32f5E
	movb	%al, 199(%rsp)
	.loc	41 0 20
	movq	_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hd2673ece5df91901E@GOTPCREL(%rip), %rsi
	.loc	41 16 5
	leaq	199(%rsp), %rax
	movq	%rax, 184(%rsp)
	movq	184(%rsp), %rax
	movq	%rax, 536(%rsp)
.Ltmp716:
	.loc	41 16 5
	movq	%rax, %rdi
	callq	_ZN4core3fmt10ArgumentV13new17hd754bd8a69d22ee8E
	movq	%rax, 88(%rsp)
	movq	%rdx, 80(%rsp)
	.loc	41 0 5
	leaq	.L__unnamed_18(%rip), %rax
	movq	88(%rsp), %rcx
	.loc	41 16 5
	movq	%rcx, 168(%rsp)
	movq	80(%rsp), %rdx
	movq	%rdx, 176(%rsp)
.Ltmp717:
	.loc	41 16 5
	leaq	168(%rsp), %rsi
	leaq	120(%rsp), %rdi
	movq	%rsi, 72(%rsp)
	movq	%rax, %rsi
	movl	$2, %edx
	movq	72(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117hd958e3b7230f7202E
	leaq	120(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h0d31d4b9faa6e1ecE@GOTPCREL(%rip)
	.loc	41 17 20 is_stmt 1
	leaq	304(%rsp), %rdi
	leaq	104(%rsp), %rsi
	callq	_ZN9byteorder2io12ReadBytesExt7read_u817h564428688e309ffcE
	.loc	41 0 20 is_stmt 0
	leaq	.L__unnamed_19(%rip), %rax
	.loc	41 17 20
	leaq	304(%rsp), %rdi
	movq	%rax, %rsi
	callq	_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h34cf537a266a32f5E
	movb	%al, 303(%rsp)
	.loc	41 0 20
	movq	_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hd2673ece5df91901E@GOTPCREL(%rip), %rsi
	.loc	41 17 5
	leaq	303(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	288(%rsp), %rax
	movq	%rax, 544(%rsp)
.Ltmp718:
	.loc	41 17 5
	movq	%rax, %rdi
	callq	_ZN4core3fmt10ArgumentV13new17hd754bd8a69d22ee8E
	movq	%rax, 64(%rsp)
	movq	%rdx, 56(%rsp)
	.loc	41 0 5
	leaq	.L__unnamed_18(%rip), %rax
	movq	64(%rsp), %rcx
	.loc	41 17 5
	movq	%rcx, 272(%rsp)
	movq	56(%rsp), %rdx
	movq	%rdx, 280(%rsp)
.Ltmp719:
	.loc	41 17 5
	leaq	272(%rsp), %rsi
	leaq	224(%rsp), %rdi
	movq	%rsi, 48(%rsp)
	movq	%rax, %rsi
	movl	$2, %edx
	movq	48(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117hd958e3b7230f7202E
	leaq	224(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h0d31d4b9faa6e1ecE@GOTPCREL(%rip)
	.loc	41 18 20 is_stmt 1
	leaq	408(%rsp), %rdi
	leaq	104(%rsp), %rsi
	callq	_ZN9byteorder2io12ReadBytesExt7read_u817h564428688e309ffcE
	.loc	41 0 20 is_stmt 0
	leaq	.L__unnamed_20(%rip), %rax
	.loc	41 18 20
	leaq	408(%rsp), %rdi
	movq	%rax, %rsi
	callq	_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h34cf537a266a32f5E
	movb	%al, 407(%rsp)
	.loc	41 0 20
	movq	_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hd2673ece5df91901E@GOTPCREL(%rip), %rsi
	.loc	41 18 5
	leaq	407(%rsp), %rax
	movq	%rax, 392(%rsp)
	movq	392(%rsp), %rax
	movq	%rax, 552(%rsp)
.Ltmp720:
	.loc	41 18 5
	movq	%rax, %rdi
	callq	_ZN4core3fmt10ArgumentV13new17hd754bd8a69d22ee8E
	movq	%rax, 40(%rsp)
	movq	%rdx, 32(%rsp)
	.loc	41 0 5
	leaq	.L__unnamed_18(%rip), %rax
	movq	40(%rsp), %rcx
	.loc	41 18 5
	movq	%rcx, 376(%rsp)
	movq	32(%rsp), %rdx
	movq	%rdx, 384(%rsp)
.Ltmp721:
	.loc	41 18 5
	leaq	376(%rsp), %rsi
	leaq	328(%rsp), %rdi
	movq	%rsi, 24(%rsp)
	movq	%rax, %rsi
	movl	$2, %edx
	movq	24(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117hd958e3b7230f7202E
	leaq	328(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h0d31d4b9faa6e1ecE@GOTPCREL(%rip)
	.loc	41 19 20 is_stmt 1
	leaq	512(%rsp), %rdi
	leaq	104(%rsp), %rsi
	callq	_ZN9byteorder2io12ReadBytesExt7read_u817h564428688e309ffcE
	.loc	41 0 20 is_stmt 0
	leaq	.L__unnamed_21(%rip), %rax
	.loc	41 19 20
	leaq	512(%rsp), %rdi
	movq	%rax, %rsi
	callq	_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h34cf537a266a32f5E
	movb	%al, 511(%rsp)
	.loc	41 0 20
	movq	_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hd2673ece5df91901E@GOTPCREL(%rip), %rsi
	.loc	41 19 5
	leaq	511(%rsp), %rax
	movq	%rax, 496(%rsp)
	movq	496(%rsp), %rax
	movq	%rax, 560(%rsp)
.Ltmp722:
	.loc	41 19 5
	movq	%rax, %rdi
	callq	_ZN4core3fmt10ArgumentV13new17hd754bd8a69d22ee8E
	movq	%rax, 16(%rsp)
	movq	%rdx, 8(%rsp)
	.loc	41 0 5
	leaq	.L__unnamed_18(%rip), %rax
	movq	16(%rsp), %rcx
	.loc	41 19 5
	movq	%rcx, 480(%rsp)
	movq	8(%rsp), %rdx
	movq	%rdx, 488(%rsp)
.Ltmp723:
	.loc	41 19 5
	leaq	480(%rsp), %rsi
	leaq	432(%rsp), %rdi
	movq	%rsi, (%rsp)
	movq	%rax, %rsi
	movl	$2, %edx
	movq	(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117hd958e3b7230f7202E
	leaq	432(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h0d31d4b9faa6e1ecE@GOTPCREL(%rip)
.Ltmp724:
	.loc	41 20 2 is_stmt 1
	addq	$568, %rsp
	.cfi_def_cfa_offset 8
	retq
.Ltmp725:
.Lfunc_end179:
	.size	_ZN6cursor4main17h7a95d388df9093a0E, .Lfunc_end179-_ZN6cursor4main17h7a95d388df9093a0E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
.Lfunc_begin180:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movb	__rustc_debug_gdb_scripts_section__(%rip), %al
	movslq	%edi, %rcx
	leaq	_ZN6cursor4main17h7a95d388df9093a0E(%rip), %rdi
	movq	%rsi, 16(%rsp)
	movq	%rcx, %rsi
	movq	16(%rsp), %rdx
	movb	%al, 15(%rsp)
	callq	_ZN3std2rt10lang_start17hb31d7644e3c573daE
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end180:
	.size	main, .Lfunc_end180-main
	.cfi_endproc
