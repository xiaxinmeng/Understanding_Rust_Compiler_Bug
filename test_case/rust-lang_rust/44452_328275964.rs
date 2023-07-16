rust
.text
	.intel_syntax noprefix
	.file	"playground.cgu-0.rs"
	.section	".text.cold._ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE,@function
_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE:
	.cfi_startproc
	push	r14
.Lcfi0:
	.cfi_def_cfa_offset 16
	push	rbx
.Lcfi1:
	.cfi_def_cfa_offset 24
	sub	rsp, 72
.Lcfi2:
	.cfi_def_cfa_offset 96
.Lcfi3:
	.cfi_offset rbx, -24
.Lcfi4:
	.cfi_offset r14, -16
	mov	r14, rdi
	mov	rbx, qword ptr [r14 + 8]
	test	rbx, rbx
	je	.LBB0_6
	lea	rsi, [4*rbx]
	lea	rcx, [8*rbx]
	mov	rdi, qword ptr [r14]
	lea	r9, [rsp + 8]
	mov	edx, 4
	mov	r8d, 4
	call	__rust_realloc@PLT
	test	rax, rax
	je	.LBB0_4
	add	rbx, rbx
	jmp	.LBB0_3
.LBB0_6:
	lea	rdx, [rsp + 8]
	mov	edi, 16
	mov	esi, 4
	call	__rust_alloc@PLT
	test	rax, rax
	je	.LBB0_8
	mov	ebx, 4
.LBB0_3:
	mov	qword ptr [r14], rax
	mov	qword ptr [r14 + 8], rbx
	add	rsp, 72
	pop	rbx
	pop	r14
	ret
.LBB0_4:
	mov	rax, qword ptr [rsp + 8]
	movups	xmm0, xmmword ptr [rsp + 16]
	movaps	xmmword ptr [rsp + 32], xmm0
	mov	qword ptr [rsp + 8], rax
	movaps	xmm0, xmmword ptr [rsp + 32]
	jmp	.LBB0_5
.LBB0_8:
	movups	xmm0, xmmword ptr [rsp + 16]
	movaps	xmmword ptr [rsp + 32], xmm0
	movaps	xmm0, xmmword ptr [rsp + 32]
	movaps	xmmword ptr [rsp + 48], xmm0
	movaps	xmm0, xmmword ptr [rsp + 48]
.LBB0_5:
	movups	xmmword ptr [rsp + 16], xmm0
	lea	rdi, [rsp + 8]
	call	_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E
.Lfunc_end0:
	.size	_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE, .Lfunc_end0-_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE
	.cfi_endproc

	.section	.text._ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E,@function
_ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E:
	.cfi_startproc
	mov	rsi, qword ptr [rdi + 8]
	test	rsi, rsi
	je	.LBB1_1
	mov	rdi, qword ptr [rdi]
	shl	rsi, 2
	mov	edx, 4
	jmp	__rust_dealloc@PLT
.LBB1_1:
	ret
.Lfunc_end1:
	.size	_ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E, .Lfunc_end1-_ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E
	.cfi_endproc

	.section	".text.cold._ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E,@function
_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E:
	.cfi_startproc
	sub	rsp, 24
.Lcfi5:
	.cfi_def_cfa_offset 32
	mov	rax, qword ptr [rdi + 16]
	mov	qword ptr [rsp + 16], rax
	movups	xmm0, xmmword ptr [rdi]
	movaps	xmmword ptr [rsp], xmm0
	mov	rdi, rsp
	call	__rust_oom@PLT
.Lfunc_end2:
	.size	_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E, .Lfunc_end2-_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$3oom17h8b5f820f2dd0e667E
	.cfi_endproc

	.section	.text._ZN10playground4main17hf21e17cd5a16d5d9E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN10playground4main17hf21e17cd5a16d5d9E,@function
_ZN10playground4main17hf21e17cd5a16d5d9E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	push	rbx
.Lcfi6:
	.cfi_def_cfa_offset 16
	sub	rsp, 32
.Lcfi7:
	.cfi_def_cfa_offset 48
.Lcfi8:
	.cfi_offset rbx, -16
	mov	qword ptr [rsp + 8], 4
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 16], xmm0
.Ltmp0:
	lea	rdi, [rsp + 8]
	call	_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h1882b1047d8074ffE
.Ltmp1:
	mov	rdi, qword ptr [rsp + 8]
	mov	rax, qword ptr [rsp + 24]
	mov	dword ptr [rdi + 4*rax], 0
	inc	rax
	mov	qword ptr [rsp + 24], rax
	je	.LBB3_2
	mov	rsi, qword ptr [rsp + 16]
	test	rsi, rsi
	je	.LBB3_6
	shl	rsi, 2
	mov	edx, 4
	call	__rust_dealloc@PLT
.LBB3_6:
	add	rsp, 32
	pop	rbx
	ret
.LBB3_2:
.Ltmp2:
	lea	rdi, [rip + panic_bounds_check_loc.2]
	xor	esi, esi
	xor	edx, edx
	call	_ZN4core9panicking18panic_bounds_check17h78beadfd8229dc37E@PLT
.Ltmp3:
.LBB3_7:
.Ltmp4:
	mov	rbx, rax
	lea	rdi, [rsp + 8]
	call	_ZN4core3ptr13drop_in_place17h62d05f1818f0f9d8E
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.Lfunc_end3:
	.size	_ZN10playground4main17hf21e17cd5a16d5d9E, .Lfunc_end3-_ZN10playground4main17hf21e17cd5a16d5d9E
	.cfi_endproc
	.section	.gcc_except_table,"a",@progbits
	.p2align	2
GCC_except_table3:
.Lexception0:
	.byte	255
	.byte	155
	.asciz	"\234"
	.byte	3
	.byte	26
	.long	.Ltmp0-.Lfunc_begin0
	.long	.Ltmp3-.Ltmp0
	.long	.Ltmp4-.Lfunc_begin0
	.byte	0
	.long	.Ltmp3-.Lfunc_begin0
	.long	.Lfunc_end3-.Ltmp3
	.long	0
	.byte	0
	.p2align	2

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	mov	rax, rsi
	mov	rcx, rdi
	lea	rdi, [rip + _ZN10playground4main17hf21e17cd5a16d5d9E]
	mov	rsi, rcx
	mov	rdx, rax
	jmp	_ZN3std2rt10lang_start17h573cecb903a42a26E@PLT
.Lfunc_end4:
	.size	main, .Lfunc_end4-main
	.cfi_endproc

	.type	str.1,@object
	.section	.rodata.str.1,"a",@progbits
	.p2align	4
str.1:
	.ascii	"/checkout/src/liballoc/vec.rs"
	.size	str.1, 29

	.type	panic_bounds_check_loc.2,@object
	.section	.data.rel.ro.panic_bounds_check_loc.2,"aw",@progbits
	.p2align	3
panic_bounds_check_loc.2:
	.quad	str.1
	.quad	29
	.long	1555
	.long	10
	.size	panic_bounds_check_loc.2, 24

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality

	.section	".note.GNU-stack","",@progbits
