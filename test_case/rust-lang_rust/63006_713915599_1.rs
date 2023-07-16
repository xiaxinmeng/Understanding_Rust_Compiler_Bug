asm
[snip constants]

playground::original:
	movq	%rdi, %rax
	movaps	.LCPI0_0(%rip), %xmm0
	movups	%xmm0, (%rdi)
	movaps	.LCPI0_1(%rip), %xmm0
	movups	%xmm0, 16(%rdi)
	movaps	.LCPI0_2(%rip), %xmm0
	movups	%xmm0, 32(%rdi)
	movaps	.LCPI0_3(%rip), %xmm0
	movups	%xmm0, 48(%rdi)
	movaps	.LCPI0_4(%rip), %xmm0
	movups	%xmm0, 64(%rdi)
	movaps	.LCPI0_5(%rip), %xmm0
	movups	%xmm0, 80(%rdi)
	movaps	.LCPI0_6(%rip), %xmm0
	movups	%xmm0, 96(%rdi)
	movaps	.LCPI0_7(%rip), %xmm0
	movups	%xmm0, 112(%rdi)
	retq
