
/t/r $ cat test.rs
pub fn square(num: i32) -> i32 {
  num * num
}
/t/r $ rustc --version
rustc 1.14.0 (e8a012324 2016-12-16)
/t/r $ rustc -O -C debuginfo=0 --emit asm test.rs --crate-type rlib
/t/r $ cat test.s
...
	.cfi_startproc
	imull	%edi, %edi
	movl	%edi, %eax
	retq
.Lfunc_end0:
	.size	_ZN4test6square17h3872c4fac15ac3c4E, .Lfunc_end0-_ZN4test6square17h3872c4fac15ac3c4E
	.cfi_endproc
...

/t/r $ rustc -O -C debuginfo=1 --emit asm test.rs --crate-type rlib
...
	.cfi_startproc
	pushq	%rbp
.Ltmp0:
	.cfi_def_cfa_offset 16
.Ltmp1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
.Ltmp2:
	.cfi_def_cfa_register %rbp
.Ltmp3:
	.loc	1 2 0 prologue_end
	imull	%edi, %edi
	.loc	1 3 0
	movl	%edi, %eax
	popq	%rbp
	retq
.Ltmp4:
.Lfunc_end0:
	.size	_ZN4test6square17h3872c4fac15ac3c4E, .Lfunc_end0-_ZN4test6square17h3872c4fac15ac3c4E
	.cfi_endproc
...
