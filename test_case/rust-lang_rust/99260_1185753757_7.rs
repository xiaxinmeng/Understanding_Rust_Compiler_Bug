diff
--- unwind.s	2022-07-15 13:41:59.629266400 -0400
+++ abort.s	2022-07-15 13:42:09.911164300 -0400
@@ -19,103 +19,61 @@
 	.seh_handler __CxxFrameHandler3, @unwind, @except
 	pushq	%rbp
 	.seh_pushreg %rbp
-	subq	$80, %rsp
-	.seh_stackalloc 80
-	leaq	80(%rsp), %rbp
-	.seh_setframe %rbp, 80
+	pushq	%rsi
+	.seh_pushreg %rsi
+	subq	$40, %rsp
+	.seh_stackalloc 40
+	leaq	32(%rsp), %rbp
+	.seh_setframe %rbp, 32
 	.seh_endprologue
-	movq	$-2, -16(%rbp)
+	movq	$-2, (%rbp)
 	movq	__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h832a50073c85f225E(%rip), %rax
 	movq	(%rax), %rax
 	shlq	$1, %rax
 	testq	%rax, %rax
 	jne	.LBB0_2
-	movl	$0, -20(%rbp)
+	xorl	%esi, %esi
 .LBB0_3:
 .Ltmp0:
 	callq	happy
 .Ltmp1:
-	movb	$1, %al
-	movq	%rax, -32(%rbp)
-	movl	$42, %eax
-	movq	%rax, -40(%rbp)
-.LBB0_5:
-$ehgcr_0_5:
-	cmpb	$0, -20(%rbp)
+	testb	%sil, %sil
 	je	.LBB0_6
 	callq	_ZN3std9panicking11panic_count8increase17h424b40195d3e6f0fE
 .LBB0_6:
-	cmpb	$0, -32(%rbp)
-	movq	-40(%rbp), %rax
-	jne	.LBB0_8
-	callq	unfortunate
-	movl	$13, %eax
-.LBB0_8:
-	addq	$80, %rsp
+	movl	$42, %eax
+	addq	$40, %rsp
+	popq	%rsi
 	popq	%rbp
 	retq
 .LBB0_2:
 	callq	_ZN3std9panicking11panic_count22try_decrease_slow_path17hf1721c3fb8e2c7e3E
-	movl	%eax, -20(%rbp)
+	movl	%eax, %esi
 	jmp	.LBB0_3
 	.seh_handlerdata
 	.long	($cppxdata$notable_function)@IMGREL
 	.section	.text,"xr",one_only,notable_function
 	.seh_endproc
-	.def	"?catch$9@?0?notable_function@4HA";
+	.def	"?catch$7@?0?notable_function@4HA";
 	.scl	3;
 	.type	32;
 	.endef
 	.p2align	4, 0x90
-"?catch$9@?0?notable_function@4HA":
-.seh_proc "?catch$9@?0?notable_function@4HA"
+"?catch$7@?0?notable_function@4HA":
+.seh_proc "?catch$7@?0?notable_function@4HA"
 	.seh_handler __CxxFrameHandler3, @unwind, @except
-.LBB0_9:
+.LBB0_7:
 	movq	%rdx, 16(%rsp)
 	pushq	%rbp
 	.seh_pushreg %rbp
-	subq	$32, %rsp
-	.seh_stackalloc 32
-	leaq	80(%rdx), %rbp
+	pushq	%rsi
+	.seh_pushreg %rsi
+	subq	$40, %rsp
+	.seh_stackalloc 40
+	leaq	32(%rdx), %rbp
 	.seh_endprologue
-	movq	-8(%rbp), %rcx
-	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
-	movq	%rax, -40(%rbp)
-	xorl	%eax, %eax
-	movq	%rax, -32(%rbp)
-	leaq	.LBB0_5(%rip), %rax
-	addq	$32, %rsp
-	popq	%rbp
-	retq
-	.seh_handlerdata
-	.long	($cppxdata$notable_function)@IMGREL
-	.section	.text,"xr",one_only,notable_function
-	.seh_endproc
-	.def	"?catch$10@?0?notable_function@4HA";
-	.scl	3;
-	.type	32;
-	.endef
-	.p2align	4, 0x90
-"?catch$10@?0?notable_function@4HA":
-.seh_proc "?catch$10@?0?notable_function@4HA"
-	.seh_handler __CxxFrameHandler3, @unwind, @except
-.LBB0_10:
-	movq	%rdx, 16(%rsp)
-	pushq	%rbp
-	.seh_pushreg %rbp
-	subq	$32, %rsp
-	.seh_stackalloc 32
-	leaq	80(%rdx), %rbp
-	.seh_endprologue
-	xorl	%eax, %eax
-	movq	%rax, -32(%rbp)
-	xorl	%ecx, %ecx
-	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
-	movq	%rax, -40(%rbp)
-	leaq	.LBB0_5(%rip), %rax
-	addq	$32, %rsp
-	popq	%rbp
-	retq
+	callq	_ZN4core9panicking15panic_no_unwind17h49e55cbf27726099E
+	ud2
 .Lfunc_end0:
 	.seh_handlerdata
 	.long	($cppxdata$notable_function)@IMGREL
@@ -129,9 +87,9 @@
 	.long	($stateUnwindMap$notable_function)@IMGREL
 	.long	1
 	.long	($tryMap$notable_function)@IMGREL
-	.long	5
+	.long	4
 	.long	($ip2state$notable_function)@IMGREL
-	.long	64
+	.long	32
 	.long	0
 	.long	1
 $stateUnwindMap$notable_function:
@@ -143,19 +101,14 @@
 	.long	0
 	.long	0
 	.long	1
-	.long	2
+	.long	1
 	.long	($handlerMap$0$notable_function)@IMGREL
 $handlerMap$0$notable_function:
-	.long	8
-	.long	__rust_panic_type_info@IMGREL
-	.long	72
-	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
-	.long	56
 	.long	64
 	.long	0
 	.long	0
-	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
-	.long	56
+	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
+	.long	72
 $ip2state$notable_function:
 	.long	.Lfunc_begin0@IMGREL
 	.long	-1
@@ -163,17 +116,6 @@
 	.long	0
 	.long	.Ltmp1@IMGREL+1
 	.long	-1
-	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
-	.long	1
-	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
+	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
 	.long	1
 	.section	.text,"xr",one_only,notable_function
-
-	.section	.data,"dw",discard,__rust_panic_type_info
-	.globl	__rust_panic_type_info
-	.p2align	4
-__rust_panic_type_info:
-	.quad	"??_7type_info@@6B@"
-	.quad	0
-	.asciz	"rust_panic"
-	.zero	5
