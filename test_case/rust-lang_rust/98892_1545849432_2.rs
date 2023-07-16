asm
	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"swaps.2c53fb8c2b847967-cgu.0"
	.def	swap_strings;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_strings
	.globl	swap_strings
	.p2align	4, 0x90
swap_strings:
	movups	(%rcx), %xmm0
	movups	(%rdx), %xmm1
	movups	%xmm1, (%rcx)
	movups	%xmm0, (%rdx)
	movq	16(%rcx), %rax
	movq	16(%rdx), %r8
	movq	%r8, 16(%rcx)
	movq	%rax, 16(%rdx)
	retq

	.def	swap_another;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_another
	.globl	swap_another
	.p2align	4, 0x90
swap_another:
	movq	(%rcx), %rax
	movq	(%rdx), %r8
	movq	%r8, (%rcx)
	movq	%rax, (%rdx)
	movl	8(%rcx), %eax
	movl	8(%rdx), %r8d
	movl	%r8d, 8(%rcx)
	movl	%eax, 8(%rdx)
	retq

	.def	swap_large_packed;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_large_packed
	.globl	swap_large_packed
	.p2align	4, 0x90
swap_large_packed:
	movups	(%rcx), %xmm0
	movups	16(%rcx), %xmm1
	movups	(%rdx), %xmm2
	movups	16(%rdx), %xmm3
	movups	%xmm3, 16(%rcx)
	movups	%xmm2, (%rcx)
	movups	%xmm0, (%rdx)
	movups	%xmm1, 16(%rdx)
	movzbl	40(%rdx), %eax
	movzbl	40(%rcx), %r8d
	movq	32(%rdx), %r9
	movq	32(%rcx), %r10
	movq	%r9, 32(%rcx)
	movq	%r10, 32(%rdx)
	movb	%al, 40(%rcx)
	movb	%r8b, 40(%rdx)
	retq

	.def	swap_small_packed;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_small_packed
	.globl	swap_small_packed
	.p2align	4, 0x90
swap_small_packed:
	movq	(%rcx), %rax
	movq	(%rdx), %r8
	movq	%r8, (%rcx)
	movq	%rax, (%rdx)
	movzbl	8(%rcx), %eax
	movzbl	8(%rdx), %r8d
	movb	%r8b, 8(%rcx)
	movb	%al, 8(%rdx)
	retq
