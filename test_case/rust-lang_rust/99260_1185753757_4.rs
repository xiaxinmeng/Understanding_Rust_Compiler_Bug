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
	pushq	%rsi
	.seh_pushreg %rsi
	subq	$40, %rsp
	.seh_stackalloc 40
	leaq	32(%rsp), %rbp
	.seh_setframe %rbp, 32
	.seh_endprologue
	movq	$-2, (%rbp)
	movq	__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h832a50073c85f225E(%rip), %rax
	movq	(%rax), %rax
	shlq	$1, %rax
	testq	%rax, %rax
	jne	.LBB0_2
	xorl	%esi, %esi
.LBB0_3:
.Ltmp0:
	callq	happy
.Ltmp1:
	testb	%sil, %sil
	je	.LBB0_6
	callq	_ZN3std9panicking11panic_count8increase17h424b40195d3e6f0fE
.LBB0_6:
	movl	$42, %eax
	addq	$40, %rsp
	popq	%rsi
	popq	%rbp
	retq
.LBB0_2:
	callq	_ZN3std9panicking11panic_count22try_decrease_slow_path17hf1721c3fb8e2c7e3E
	movl	%eax, %esi
	jmp	.LBB0_3
	.seh_handlerdata
	.long	($cppxdata$notable_function)@IMGREL
	.section	.text,"xr",one_only,notable_function
	.seh_endproc
	.def	"?catch$7@?0?notable_function@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?catch$7@?0?notable_function@4HA":
.seh_proc "?catch$7@?0?notable_function@4HA"
	.seh_handler __CxxFrameHandler3, @unwind, @except
.LBB0_7:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	pushq	%rsi
	.seh_pushreg %rsi
	subq	$40, %rsp
	.seh_stackalloc 40
	leaq	32(%rdx), %rbp
	.seh_endprologue
	callq	_ZN4core9panicking15panic_no_unwind17h49e55cbf27726099E
	ud2
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
	.long	4
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
	.long	1
	.long	($handlerMap$0$notable_function)@IMGREL
$handlerMap$0$notable_function:
	.long	64
	.long	0
	.long	0
	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
	.long	72
$ip2state$notable_function:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
	.long	1
	.section	.text,"xr",one_only,notable_function
