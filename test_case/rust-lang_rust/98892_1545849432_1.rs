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
.seh_proc swap_strings
	subq	$24, %rsp
	.seh_stackalloc 24
	.seh_endprologue
	movq	16(%rcx), %rax
	movq	%rax, 16(%rsp)
	movups	(%rcx), %xmm0
	movaps	%xmm0, (%rsp)
	movq	16(%rdx), %rax
	movq	%rax, 16(%rcx)
	movups	(%rdx), %xmm0
	movups	%xmm0, (%rcx)
	movq	16(%rsp), %rax
	movq	%rax, 16(%rdx)
	movaps	(%rsp), %xmm0
	movups	%xmm0, (%rdx)
	addq	$24, %rsp
	retq
	.seh_endproc

	.def	swap_another;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_another
	.globl	swap_another
	.p2align	4, 0x90
swap_another:
.seh_proc swap_another
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

	.def	swap_large_packed;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_large_packed
	.globl	swap_large_packed
	.p2align	4, 0x90
swap_large_packed:
	movups	(%rcx), %xmm0
	movups	(%rdx), %xmm1
	movups	%xmm1, (%rcx)
	movups	%xmm0, (%rdx)
	movups	16(%rcx), %xmm0
	movups	16(%rdx), %xmm1
	movups	%xmm1, 16(%rcx)
	movups	%xmm0, 16(%rdx)
	movq	32(%rcx), %rax
	movq	32(%rdx), %r8
	movq	%r8, 32(%rcx)
	movq	%rax, 32(%rdx)
	movzbl	40(%rcx), %eax
	movzbl	40(%rdx), %r8d
	movb	%r8b, 40(%rcx)
	movb	%al, 40(%rdx)
	retq

	.def	swap_small_packed;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,swap_small_packed
	.globl	swap_small_packed
	.p2align	4, 0x90
swap_small_packed:
	movzbl	(%rcx), %eax
	movzbl	(%rdx), %r8d
	movb	%r8b, (%rcx)
	movb	%al, (%rdx)
	movzbl	1(%rcx), %eax
	movzbl	1(%rdx), %r8d
	movb	%r8b, 1(%rcx)
	movb	%al, 1(%rdx)
	movzbl	2(%rcx), %eax
	movzbl	2(%rdx), %r8d
	movb	%r8b, 2(%rcx)
	movb	%al, 2(%rdx)
	movzbl	3(%rcx), %eax
	movzbl	3(%rdx), %r8d
	movb	%r8b, 3(%rcx)
	movb	%al, 3(%rdx)
	movzbl	4(%rcx), %eax
	movzbl	4(%rdx), %r8d
	movb	%r8b, 4(%rcx)
	movb	%al, 4(%rdx)
	movzbl	5(%rcx), %eax
	movzbl	5(%rdx), %r8d
	movb	%r8b, 5(%rcx)
	movb	%al, 5(%rdx)
	movzbl	6(%rcx), %eax
	movzbl	6(%rdx), %r8d
	movb	%r8b, 6(%rcx)
	movb	%al, 6(%rdx)
	movzbl	7(%rcx), %eax
	movzbl	7(%rdx), %r8d
	movb	%r8b, 7(%rcx)
	movb	%al, 7(%rdx)
	movzbl	8(%rcx), %eax
	movzbl	8(%rdx), %r8d
	movb	%r8b, 8(%rcx)
	movb	%al, 8(%rdx)
	retq
