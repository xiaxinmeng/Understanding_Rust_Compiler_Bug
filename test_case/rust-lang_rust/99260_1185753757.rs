asm
	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"lib.4978488e-cgu.0"
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
	subq	$48, %rsp
	.seh_stackalloc 48
	leaq	48(%rsp), %rbp
	.seh_setframe %rbp, 48
	.seh_endprologue
	movq	$-2, -16(%rbp)
.Ltmp0:
	callq	happy
.Ltmp1:
	movl	$42, %eax
.LBB0_5:
	addq	$48, %rsp
	popq	%rbp
	retq
.LBB0_4:
$ehgcr_0_4:
	callq	unfortunate
	movl	$13, %eax
	jmp	.LBB0_5
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.def	"?catch$2@?0?notable_function@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?catch$2@?0?notable_function@4HA":
.seh_proc "?catch$2@?0?notable_function@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB0_2:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	48(%rdx), %rbp
	.seh_endprologue
	movq	-8(%rbp), %rcx
	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
	leaq	.LBB0_4(%rip), %rax
	addq	$32, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.def	"?catch$3@?0?notable_function@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?catch$3@?0?notable_function@4HA":
.seh_proc "?catch$3@?0?notable_function@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB0_3:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	48(%rdx), %rbp
	.seh_endprologue
	xorl	%ecx, %ecx
	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
	leaq	.LBB0_4(%rip), %rax
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
	.long	32
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
	.long	40
	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
	.long	56
	.long	64
	.long	0
	.long	0
	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
	.long	56
$ip2state$notable_function:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
	.long	1
	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
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
