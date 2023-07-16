asm
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap17hdbd5a86115aeeb48E:
	.cfi_startproc
	cmpl	$0, (%rdi)
	je	LBB0_2
	movzbl	8(%rdi), %ecx
	shlq	$32, %rcx
	movl	4(%rdi), %eax
	orq	%rcx, %rax
	retq
LBB0_2:
	pushq	%rbp
Lcfi0:
	.cfi_def_cfa_offset 16
Lcfi1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi2:
	.cfi_def_cfa_register %rbp
	movq	__ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap14_MSG_FILE_LINE17he9e3266a381f7b99E@GOTPCREL(%rip), %rdi
	callq	__ZN4core9panicking5panic17hafbe89720e5223c3E
	.cfi_endproc

	.p2align	4, 0x90
__ZN4core3ptr13drop_in_place17h187871343964cdfcE:
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	movq	(%rbx), %rdi
	callq	__ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE
	movq	(%rbx), %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	jmp	__ZN5alloc4heap8box_free17h52fcf778f557f0b4E

	.p2align	4, 0x90
__ZN4core3ptr13drop_in_place17h31d8df590941a85fE:
	pushq	%rbp
	movq	%rsp, %rbp
	cmpq	$0, (%rdi)
	je	LBB2_1
	addq	$8, %rdi
	popq	%rbp
	jmp	__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E
LBB2_1:
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE:
	pushq	%rbp
	movq	%rsp, %rbp
	movb	(%rdi), %al
	testb	%al, %al
	je	LBB3_3
	cmpb	$1, %al
	jne	LBB3_2
LBB3_3:
	addq	$8, %rdi
	popq	%rbp
	jmp	__ZN4core3ptr13drop_in_place17h187871343964cdfcE
LBB3_2:
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E:
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	cmpb	$0, (%rbx)
	je	LBB4_1
	leaq	8(%rbx), %rdi
	addq	$24, %rbx
	callq	__ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE
	movq	%rbx, %rdi
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	jmp	__ZN4core3ptr13drop_in_place17h48c9e5bc21e188aaE
LBB4_1:
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E:
	pushq	%rbp
	movq	%rsp, %rbp
	movabsq	$1099511627775, %rax
	andq	%rdi, %rax
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN5alloc4heap10deallocate17h0cf606d4dc6542b7E:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$16, %esi
	movl	$8, %edx
	popq	%rbp
	jmp	___rust_deallocate

	.p2align	4, 0x90
__ZN5alloc4heap8box_free17h52fcf778f557f0b4E:
	pushq	%rbp
	movq	%rsp, %rbp
	popq	%rbp
	jmp	__ZN5alloc4heap10deallocate17h0cf606d4dc6542b7E

	.p2align	4, 0x90
__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$1, (%rdi)
	movl	%esi, 4(%rdi)
	shrq	$32, %rsi
	movb	%sil, 8(%rdi)
	movq	%rdi, %rax
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$12from_success17h4169241993d9016dE:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$0, (%rdi)
	movq	%rdi, %rax
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E:
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%rbx
	pushq	%rax
	movq	%rdi, %rbx
	cmpl	$0, (%rsi)
	je	LBB10_1
	movzbl	8(%rsi), %eax
	shlq	$32, %rax
	movl	4(%rsi), %esi
	orq	%rax, %rsi
	movq	%rbx, %rdi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E
	jmp	LBB10_2
LBB10_1:
	movq	%rbx, %rdi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$12from_success17h4169241993d9016dE
LBB10_2:
	movq	%rbx, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq

	.p2align	4, 0x90
__ZN2_14main17hb2ffcf74c477bad3E:
	.cfi_startproc
	pushq	%rbp
Lcfi3:
	.cfi_def_cfa_offset 16
Lcfi4:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi5:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	leaq	-16(%rbp), %rdi
	callq	__ZN2_11g17h62cbbaab7a2481a3E
	addq	$16, %rsp
	popq	%rbp
	retq
	.cfi_endproc

	.p2align	4, 0x90
__ZN2_11g17h62cbbaab7a2481a3E:
	.cfi_startproc
	pushq	%rbp
Lcfi6:
	.cfi_def_cfa_offset 16
Lcfi7:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi8:
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	subq	$136, %rsp
Lcfi9:
	.cfi_offset %rbx, -56
Lcfi10:
	.cfi_offset %r12, -48
Lcfi11:
	.cfi_offset %r13, -40
Lcfi12:
	.cfi_offset %r14, -32
Lcfi13:
	.cfi_offset %r15, -24
	movq	%rdi, -104(%rbp)
	leaq	-160(%rbp), %rbx
	movb	$1, %al
	xorl	%r14d, %r14d
	leaq	-96(%rbp), %r15
	xorl	%r13d, %r13d
	testb	$1, %al
	je	LBB12_2
	jmp	LBB12_6
	.p2align	4, 0x90
LBB12_3:
	movb	$1, %r12b
	testb	$1, %r13b
	jne	LBB12_10
	jmp	LBB12_4
	.p2align	4, 0x90
LBB12_6:
	movq	$1, -168(%rbp)
	movw	$0, -160(%rbp)
	movq	32(%rbx), %rax
	movq	%rax, -64(%rbp)
	movq	24(%rbx), %rax
	movq	%rax, -72(%rbp)
	movq	16(%rbx), %rax
	movq	%rax, -80(%rbp)
	movq	(%rbx), %rax
	movq	8(%rbx), %rcx
	movq	%rcx, -88(%rbp)
	movq	%rax, -96(%rbp)
	cmpb	$0, -96(%rbp)
	je	LBB12_8
	xorl	%r12d, %r12d
	jmp	LBB12_11
	.p2align	4, 0x90
LBB12_8:
	movl	$0, -56(%rbp)
	leaq	-120(%rbp), %rdi
	leaq	-56(%rbp), %rsi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E
	cmpl	$0, -120(%rbp)
	jne	LBB12_15
	movl	$1, %r14d
	movb	$1, %r13b
	xorl	%r12d, %r12d
LBB12_10:
	cmpb	$0, -96(%rbp)
	je	LBB12_4
LBB12_11:
	movq	%r15, %rdi
	callq	__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E
	xorl	%r13d, %r13d
	cmpq	$1, -168(%rbp)
	jne	LBB12_5
	jmp	LBB12_12
	.p2align	4, 0x90
LBB12_4:
	cmpq	$1, -168(%rbp)
	jne	LBB12_5
LBB12_12:
	testb	%r12b, %r12b
	je	LBB12_1
	movq	%rbx, %rdi
	callq	__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E
	jmp	LBB12_1
	.p2align	4, 0x90
LBB12_5:
	leaq	-168(%rbp), %rdi
	callq	__ZN4core3ptr13drop_in_place17h31d8df590941a85fE
LBB12_1:
	xorl	%eax, %eax
	testb	$1, %al
	jne	LBB12_6
LBB12_2:
	movq	$0, -168(%rbp)
	movl	%r14d, -96(%rbp)
	movl	$1, -92(%rbp)
	movb	$0, -88(%rbp)
	movb	-41(%rbp), %al
	leaq	-87(%rbp), %rcx
	movb	%al, 2(%rcx)
	movzwl	-43(%rbp), %eax
	movw	%ax, (%rcx)
	movq	%r15, %rdi
	callq	__ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap17hdbd5a86115aeeb48E
	movl	$1, -96(%rbp)
	movl	%eax, -92(%rbp)
	shrq	$32, %rax
	movb	%al, -88(%rbp)
	leaq	-56(%rbp), %rdi
	movq	%r15, %rsi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$9translate17had4b200d2ffbc2e9E
	cmpl	$0, -56(%rbp)
	je	LBB12_3
	movzbl	-48(%rbp), %eax
	shlq	$32, %rax
	movl	-52(%rbp), %edi
	orq	%rax, %rdi
	callq	__ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E
	movabsq	$1099511627775, %rsi
	andq	%rax, %rsi
	movq	-104(%rbp), %rbx
	movq	%rbx, %rdi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E
	movb	$1, %r14b
	testb	$1, %r13b
	jne	LBB12_16
	jmp	LBB12_18
LBB12_15:
	movzbl	-112(%rbp), %eax
	shlq	$32, %rax
	movl	-116(%rbp), %edi
	orq	%rax, %rdi
	callq	__ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hd676c2e417a31d76E
	movabsq	$1099511627775, %rsi
	andq	%rax, %rsi
	movq	-104(%rbp), %rbx
	movq	%rbx, %rdi
	callq	__ZN77_$LT$core..result..Result$LT$U$C$$u20$V$GT$$u20$as$u20$core..ops..Carrier$GT$10from_error17h983ed059e5df6262E
	xorl	%r14d, %r14d
LBB12_16:
	cmpb	$0, -96(%rbp)
	je	LBB12_18
	leaq	-96(%rbp), %rdi
	callq	__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E
LBB12_18:
	cmpq	$1, -168(%rbp)
	jne	LBB12_21
	testb	%r14b, %r14b
	je	LBB12_22
	leaq	-160(%rbp), %rdi
	callq	__ZN4core3ptr13drop_in_place17hbf6cf7e003a035e6E
	jmp	LBB12_22
LBB12_21:
	leaq	-168(%rbp), %rdi
	callq	__ZN4core3ptr13drop_in_place17h31d8df590941a85fE
LBB12_22:
	movq	%rbx, %rax
	addq	$136, %rsp
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_main
	.p2align	4, 0x90
_main:
	.cfi_startproc
	pushq	%rbp
Lcfi14:
	.cfi_def_cfa_offset 16
Lcfi15:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Lcfi16:
	.cfi_def_cfa_register %rbp
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	__ZN2_14main17hb2ffcf74c477bad3E(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	popq	%rbp
	jmp	__ZN3std2rt10lang_start17hbda94898b2b88ae4E
	.cfi_endproc


.subsections_via_symbols
