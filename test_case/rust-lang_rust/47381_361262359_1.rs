asm
LBB99_376:
	.loc	13 467 0
	movq	%r13, %rax
	addq	$128, %rax
	movq	%r14, %rcx
	adcq	$0, %rcx
	cmpq	$256, %rax
	sbbq	$0, %rcx
	jae	LBB99_383
Ltmp3275:
	.loc	13 0 0 is_stmt 0
	xorl	%eax, %eax
	jmp	LBB99_386
LBB99_378:
Ltmp3276:
	.loc	13 468 0 is_stmt 1
	movq	%r13, %rcx
	addq	$32768, %rcx
	movq	%r14, %rdx
	adcq	$0, %rdx
	movb	$1, %al
	cmpq	$65536, %rcx
	sbbq	$0, %rdx
	jb	LBB99_381
	.loc	13 469 0
	movl	$2147483648, %ecx
	addq	%r13, %rcx
	movq	%r14, %rdx
	adcq	$0, %rdx
	shrdq	$32, %rdx, %rcx
	shrq	$32, %rdx
	movb	$2, %al
	orq	%rcx, %rdx
	je	LBB99_381
	.loc	13 470 0
	movabsq	$-9223372036854775808, %rax
	addq	%rax, %r13
	movq	%r14, %rax
	adcq	$0, %rax
	setne	%al
	addb	$3, %al
Ltmp3277:
LBB99_381:
	.loc	13 467 0
	movq	%r15, %rcx
	addq	$128, %rcx
	movq	%rdi, %rdx
	adcq	$0, %rdx
	cmpq	$256, %rcx
	movl	%esi, %r13d
	sbbq	$0, %rdx
	jae	LBB99_388
Ltmp3278:
	.loc	13 0 0 is_stmt 0
	xorl	%ecx, %ecx
	jmp	LBB99_391
LBB99_383:
Ltmp3279:
	.loc	13 468 0 is_stmt 1
	movq	%r13, %rcx
	addq	$32768, %rcx
	movq	%r14, %rdx
	adcq	$0, %rdx
	movb	$1, %al
	cmpq	$65536, %rcx
	sbbq	$0, %rdx
	jb	LBB99_386
	.loc	13 469 0
	movl	$2147483648, %ecx
	addq	%r13, %rcx
	movq	%r14, %rdx
	adcq	$0, %rdx
	shrdq	$32, %rdx, %rcx
	shrq	$32, %rdx
	movb	$2, %al
	orq	%rcx, %rdx
	je	LBB99_386
	.loc	13 470 0
	movabsq	$-9223372036854775808, %rax
	addq	%rax, %r13
	adcq	$0, %r14
	setne	%al
	addb	$3, %al

