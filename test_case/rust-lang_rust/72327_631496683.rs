assembly
_ZN10float_test4main17h3d6b0b215bf83903E:
	.cfi_startproc
	pushl	%ebx
	.cfi_def_cfa_offset 8
	subl	$24, %esp
	.cfi_def_cfa_offset 32
	.cfi_offset %ebx, -8
	calll	.L9$pb
	.cfi_adjust_cfa_offset 4
.L9$pb:
	popl	%ebx
	.cfi_adjust_cfa_offset -4
.Ltmp6:
	addl	$_GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L9$pb), %ebx
	calll	_ZN10float_test7get_num17hedf1e4650ddf3052E
	fstpl	16(%esp)
	calll	_ZN10float_test7get_num17hedf1e4650ddf3052E
	fldl	16(%esp)
	fmulp	%st, %st(1)
	fucom	%st(0)
	fnstsw	%ax
	sahf
	jp	.LBB9_5
	flds	.LCPI9_0@GOTOFF(%ebx)
	fucomp	%st(1)
	fnstsw	%ax
	sahf
	jae	.LBB9_5
	flds	.LCPI9_1@GOTOFF(%ebx)
	fxch	%st(1)
	fucom	%st(1)
	fstp	%st(1)
	fnstsw	%ax
	sahf
	jae	.LBB9_5
	fstpl	8(%esp)
	movl	12(%esp), %eax
	notl	%eax
	testl	$2146435072, %eax
	fldz
	je	.LBB9_4
.LBB9_5:
	fstp	%st(0)
	addl	$24, %esp
	.cfi_def_cfa_offset 8
	popl	%ebx
	.cfi_def_cfa_offset 4
	retl
.LBB9_4:
	.cfi_def_cfa_offset 32
	fstp	%st(0)
	calll	_ZN3std9panicking11begin_panic17h261cc8b487132e56E
	ud2

...

.LCPI9_0:
	.long	4286578688
.LCPI9_1:
	.long	2139095040
