asm
	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"swap_2_many.1b829e2e-cgu.0"
	.def	swap_2_arrays_usize;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_arrays_usize
	.globl	swap_2_arrays_usize
	.p2align	4, 0x90
swap_2_arrays_usize:
.seh_proc swap_2_arrays_usize
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	vmovups	(%rcx), %ymm0
	vmovups	%ymm0, (%rsp)
	vmovups	(%rdx), %ymm0
	vmovups	%ymm0, (%rcx)
	vmovups	(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
	addq	$40, %rsp
	vzeroupper
	retq
	.seh_endproc

	.def	swap_2_vec_deques;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_vec_deques
	.globl	swap_2_vec_deques
	.p2align	4, 0x90
swap_2_vec_deques:
.seh_proc swap_2_vec_deques
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	vmovups	(%rcx), %ymm0
	vmovups	%ymm0, (%rsp)
	vmovups	(%rdx), %ymm0
	vmovups	%ymm0, (%rcx)
	vmovups	(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
	addq	$40, %rsp
	vzeroupper
	retq
	.seh_endproc

	.def	swap_avx512_std;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_avx512_std
	.globl	swap_avx512_std
	.p2align	4, 0x90
swap_avx512_std:
	vmovaps	(%rcx), %zmm0
	vmovaps	(%rdx), %zmm1
	vmovaps	%zmm1, (%rcx)
	vmovaps	%zmm0, (%rdx)
	vzeroupper
	retq

	.def	swap_2_vecs;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_vecs
	.globl	swap_2_vecs
	.p2align	4, 0x90
swap_2_vecs:
.seh_proc swap_2_vecs
	subq	$24, %rsp
	.seh_stackalloc 24
	.seh_endprologue
	movq	16(%rcx), %rax
	movq	%rax, 16(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, (%rsp)
	movq	16(%rdx), %rax
	movq	%rax, 16(%rcx)
	vmovups	(%rdx), %xmm0
	vmovups	%xmm0, (%rcx)
	movq	16(%rsp), %rax
	movq	%rax, 16(%rdx)
	vmovaps	(%rsp), %xmm0
	vmovups	%xmm0, (%rdx)
	addq	$24, %rsp
	retq
	.seh_endproc

	.def	swap_2_arrays_2_u32;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_arrays_2_u32
	.globl	swap_2_arrays_2_u32
	.p2align	4, 0x90
swap_2_arrays_2_u32:
	movq	(%rcx), %r8
	movq	(%rdx), %rax
	movq	%rax, (%rcx)
	movq	%r8, (%rdx)
	retq

	.def	swap_2_arrays_3_u32;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_arrays_3_u32
	.globl	swap_2_arrays_3_u32
	.p2align	4, 0x90
swap_2_arrays_3_u32:
.seh_proc swap_2_arrays_3_u32
	subq	$16, %rsp
	.seh_stackalloc 16
	.seh_endprologue
	movl	8(%rcx), %eax
	movl	%eax, 8(%rsp)
	movq	(%rcx), %rax
	movq	%rax, (%rsp)
	movl	8(%rdx), %eax
	movl	%eax, 8(%rcx)
	movq	(%rdx), %rax
	movq	%rax, (%rcx)
	movl	8(%rsp), %eax
	movl	%eax, 8(%rdx)
	movq	(%rsp), %rax
	movq	%rax, (%rdx)
	addq	$16, %rsp
	retq
	.seh_endproc

	.def	swap_2_arrays_4_32;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_2_arrays_4_32
	.globl	swap_2_arrays_4_32
	.p2align	4, 0x90
swap_2_arrays_4_32:
.seh_proc swap_2_arrays_4_32
	subq	$24, %rsp
	.seh_stackalloc 24
	.seh_endprologue
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovups	(%rdx), %xmm0
	vmovups	%xmm0, (%rcx)
	vmovaps	(%rsp), %xmm0
	vmovups	%xmm0, (%rdx)
	addq	$24, %rsp
	retq
	.seh_endproc

	.globl	_fltused

