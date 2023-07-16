
foo:
	pushq	%rsi
	subq	$64, %rsp
	movq	$4, 40(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, 48(%rsp)
	leaq	40(%rsp), %rcx
	callq	_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$6double17h82e5d6919d342c2fE
	movq	40(%rsp), %rcx
	movq	56(%rsp), %rax
	movl	$0, (%rcx,%rax,4)
	addq	$1, %rax
	movq	%rax, 56(%rsp)
	je	.LBB3_2
	movl	(%rcx), %esi
	movq	48(%rsp), %rdx
	testq	%rdx, %rdx
	je	.LBB3_8
	shlq	$2, %rdx
	movl	$4, %r8d
	callq	__rust_dealloc
.LBB3_8:
	movl	%esi, %eax
	addq	$64, %rsp
	popq	%rsi
	retq
.LBB3_2:
	leaq	panic_bounds_check_loc.2(%rip), %rcx
	xorl	%edx, %edx
	xorl	%r8d, %r8d
	callq	_ZN4core9panicking18panic_bounds_check17hbf9952bd7dce1212E
	ud2
.LBB3_4:
	movq	%rax, %rcx
	callq	rust_eh_unwind_resume
	ud2
.LBB3_9:
	movq	%rax, %rsi
	leaq	40(%rsp), %rcx
	callq	_ZN4core3ptr13drop_in_place17h731ac0064b2a28c1E
	movq	%rsi, %rcx
	callq	rust_eh_unwind_resume
	ud2

