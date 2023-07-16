asm
	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"lib.68a3e9dc-cgu.0"
	.def	notable_function;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,notable_function
	.globl	notable_function
	.p2align	4, 0x90
notable_function:
.Lfunc_begin0:
.seh_proc notable_function
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$80, %rsp
	.seh_stackalloc 80
	leaq	80(%rsp), %rbp
	.seh_setframe %rbp, 80
	.seh_endprologue
	movq	$-2, -16(%rbp)
	movq	__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h832a50073c85f225E(%rip), %rax
	movq	(%rax), %rax
	shlq	$1, %rax
	testq	%rax, %rax
	jne	.LBB0_2
	movl	$0, -20(%rbp)
.LBB0_3:
.Ltmp0:
	callq	happy
.Ltmp1:
	movb	$1, %al
	movq	%rax, -32(%rbp)
	movl	$42, %eax
	movq	%rax, -40(%rbp)
.LBB0_5:
$ehgcr_0_5:
	cmpb	$0, -20(%rbp)
	je	.LBB0_6
	callq	_ZN3std9panicking11panic_count8increase17h424b40195d3e6f0fE
.LBB0_6:
	cmpb	$0, -32(%rbp)
	movq	-40(%rbp), %rax
	jne	.LBB0_8
	callq	unfortunate
	movl	$13, %eax
.LBB0_8:
	addq	$80, %rsp
	popq	%rbp
	retq
.LBB0_2:
	callq	_ZN3std9panicking11panic_count22try_decrease_slow_path17hf1721c3fb8e2c7e3E
	movl	%eax, -20(%rbp)
	jmp	.LBB0_3
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.def	"?catch$9@?0?notable_function@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?catch$9@?0?notable_function@4HA":
.seh_proc "?catch$9@?0?notable_function@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB0_9:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	80(%rdx), %rbp
	.seh_endprologue
	movq	-8(%rbp), %rcx
	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
	movq	%rax, -40(%rbp)
	xorl	%eax, %eax
	movq	%rax, -32(%rbp)
	leaq	.LBB0_5(%rip), %rax
	addq	$32, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.def	"?catch$10@?0?notable_function@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?catch$10@?0?notable_function@4HA":
.seh_proc "?catch$10@?0?notable_function@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB0_10:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	80(%rdx), %rbp
	.seh_endprologue
	xorl	%eax, %eax
	movq	%rax, -32(%rbp)
	xorl	%ecx, %ecx
	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
	movq	%rax, -40(%rbp)
	leaq	.LBB0_5(%rip), %rax
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end0:
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.section	.xdata,"dr",associative,notable_function
	.p2align	2
$cppxdata$notable_function:
	.long	429065506
	.long	2
	.long	($stateUnwindMap$notable_function)@IMGREL
	.long	1
	.long	($tryMap$notable_function)@IMGREL
	.long	5
	.long	($ip2state$notable_function)@IMGREL
	.long	64
	.long	0
	.long	1
$stateUnwindMap$notable_function:
	.long	-1
	.long	0
	.long	-1
	.long	0
$tryMap$notable_function:
	.long	0
	.long	0
	.long	1
	.long	2
	.long	($handlerMap$0$notable_function)@IMGREL
$handlerMap$0$notable_function:
	.long	8
	.long	__rust_panic_type_info@IMGREL
	.long	72
	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
	.long	56
	.long	64
	.long	0
	.long	0
	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
	.long	56
$ip2state$notable_function:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
	.long	1
	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
	.long	1
	.section	.text,"xr",one_only,notable_function

	.section	.data,"dw",discard,__rust_panic_type_info
	.globl	__rust_panic_type_info
	.p2align	4
__rust_panic_type_info:
	.quad	"??_7type_info@@6B@"
	.quad	0
	.asciz	"rust_panic"
	.zero	5
