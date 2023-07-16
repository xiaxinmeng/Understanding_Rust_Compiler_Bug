diff
--- unwind.s	2022-07-15 13:40:23.427159000 -0400
+++ abort.s	2022-07-15 13:40:17.596473500 -0400
@@ -24,20 +24,14 @@
 	leaq	48(%rsp), %rbp
 	.seh_setframe %rbp, 48
 	.seh_endprologue
-	movq	$-2, -16(%rbp)
+	movq	$-2, -8(%rbp)
 .Ltmp0:
 	callq	happy
 .Ltmp1:
 	movl	$42, %eax
-.LBB0_5:
 	addq	$48, %rsp
 	popq	%rbp
 	retq
-.LBB0_4:
-$ehgcr_0_4:
-	callq	unfortunate
-	movl	$13, %eax
-	jmp	.LBB0_5
 	.seh_handlerdata
 	.long	($cppxdata$notable_function)@IMGREL
 	.section	.text,"xr",one_only,notable_function
@@ -58,38 +52,8 @@
 	.seh_stackalloc 32
 	leaq	48(%rdx), %rbp
 	.seh_endprologue
-	movq	-8(%rbp), %rcx
-	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
-	leaq	.LBB0_4(%rip), %rax
-	addq	$32, %rsp
-	popq	%rbp
-	retq
-	.seh_handlerdata
-	.long	($cppxdata$notable_function)@IMGREL
-	.section	.text,"xr",one_only,notable_function
-	.seh_endproc
-	.def	"?catch$3@?0?notable_function@4HA";
-	.scl	3;
-	.type	32;
-	.endef
-	.p2align	4, 0x90
-"?catch$3@?0?notable_function@4HA":
-.seh_proc "?catch$3@?0?notable_function@4HA"
-	.seh_handler __CxxFrameHandler3, @unwind, @except
-.LBB0_3:
-	movq	%rdx, 16(%rsp)
-	pushq	%rbp
-	.seh_pushreg %rbp
-	subq	$32, %rsp
-	.seh_stackalloc 32
-	leaq	48(%rdx), %rbp
-	.seh_endprologue
-	xorl	%ecx, %ecx
-	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
-	leaq	.LBB0_4(%rip), %rax
-	addq	$32, %rsp
-	popq	%rbp
-	retq
+	callq	_ZN4core9panicking15panic_no_unwind17hedc8ed309bafae40E
+	ud2
 .Lfunc_end0:
 	.seh_handlerdata
 	.long	($cppxdata$notable_function)@IMGREL
@@ -103,9 +67,9 @@
 	.long	($stateUnwindMap$notable_function)@IMGREL
 	.long	1
 	.long	($tryMap$notable_function)@IMGREL
-	.long	5
+	.long	4
 	.long	($ip2state$notable_function)@IMGREL
-	.long	32
+	.long	40
 	.long	0
 	.long	1
 $stateUnwindMap$notable_function:
@@ -117,18 +81,13 @@
 	.long	0
 	.long	0
 	.long	1
-	.long	2
+	.long	1
 	.long	($handlerMap$0$notable_function)@IMGREL
 $handlerMap$0$notable_function:
-	.long	8
-	.long	__rust_panic_type_info@IMGREL
-	.long	40
-	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
-	.long	56
 	.long	64
 	.long	0
 	.long	0
-	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
+	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
 	.long	56
 $ip2state$notable_function:
 	.long	.Lfunc_begin0@IMGREL
@@ -139,15 +98,4 @@
 	.long	-1
 	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
 	.long	1
-	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
-	.long	1
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
