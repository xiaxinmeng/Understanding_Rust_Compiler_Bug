diff
--- nightly.diff	2022-07-15 13:40:58.546378000 -0400
+++ stage1.diff	2022-07-15 13:42:22.356174500 -0400
@@ -1,34 +1,96 @@
---- unwind.s	2022-07-15 13:40:23.427159000 -0400
-+++ abort.s	2022-07-15 13:40:17.596473500 -0400
-@@ -24,20 +24,14 @@
- 	leaq	48(%rsp), %rbp
- 	.seh_setframe %rbp, 48
+--- unwind.s	2022-07-15 13:41:59.629266400 -0400
++++ abort.s	2022-07-15 13:42:09.911164300 -0400
+@@ -19,103 +19,61 @@
+ 	.seh_handler __CxxFrameHandler3, @unwind, @except
+ 	pushq	%rbp
+ 	.seh_pushreg %rbp
+-	subq	$80, %rsp
+-	.seh_stackalloc 80
+-	leaq	80(%rsp), %rbp
+-	.seh_setframe %rbp, 80
++	pushq	%rsi
++	.seh_pushreg %rsi
++	subq	$40, %rsp
++	.seh_stackalloc 40
++	leaq	32(%rsp), %rbp
++	.seh_setframe %rbp, 32
  	.seh_endprologue
 -	movq	$-2, -16(%rbp)
-+	movq	$-2, -8(%rbp)
++	movq	$-2, (%rbp)
+ 	movq	__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h832a50073c85f225E(%rip), %rax
+ 	movq	(%rax), %rax
+ 	shlq	$1, %rax
+ 	testq	%rax, %rax
+ 	jne	.LBB0_2
+-	movl	$0, -20(%rbp)
++	xorl	%esi, %esi
+ .LBB0_3:
  .Ltmp0:
  	callq	happy
  .Ltmp1:
- 	movl	$42, %eax
+-	movb	$1, %al
+-	movq	%rax, -32(%rbp)
+-	movl	$42, %eax
+-	movq	%rax, -40(%rbp)
 -.LBB0_5:
- 	addq	$48, %rsp
- 	popq	%rbp
- 	retq
--.LBB0_4:
--$ehgcr_0_4:
+-$ehgcr_0_5:
+-	cmpb	$0, -20(%rbp)
++	testb	%sil, %sil
+ 	je	.LBB0_6
+ 	callq	_ZN3std9panicking11panic_count8increase17h424b40195d3e6f0fE
+ .LBB0_6:
+-	cmpb	$0, -32(%rbp)
+-	movq	-40(%rbp), %rax
+-	jne	.LBB0_8
 -	callq	unfortunate
 -	movl	$13, %eax
--	jmp	.LBB0_5
+-.LBB0_8:
+-	addq	$80, %rsp
++	movl	$42, %eax
++	addq	$40, %rsp
++	popq	%rsi
+ 	popq	%rbp
+ 	retq
+ .LBB0_2:
+ 	callq	_ZN3std9panicking11panic_count22try_decrease_slow_path17hf1721c3fb8e2c7e3E
+-	movl	%eax, -20(%rbp)
++	movl	%eax, %esi
+ 	jmp	.LBB0_3
  	.seh_handlerdata
  	.long	($cppxdata$notable_function)@IMGREL
  	.section	.text,"xr",one_only,notable_function
-@@ -58,38 +52,8 @@
- 	.seh_stackalloc 32
- 	leaq	48(%rdx), %rbp
+ 	.seh_endproc
+-	.def	"?catch$9@?0?notable_function@4HA";
++	.def	"?catch$7@?0?notable_function@4HA";
+ 	.scl	3;
+ 	.type	32;
+ 	.endef
+ 	.p2align	4, 0x90
+-"?catch$9@?0?notable_function@4HA":
+-.seh_proc "?catch$9@?0?notable_function@4HA"
++"?catch$7@?0?notable_function@4HA":
++.seh_proc "?catch$7@?0?notable_function@4HA"
+ 	.seh_handler __CxxFrameHandler3, @unwind, @except
+-.LBB0_9:
++.LBB0_7:
+ 	movq	%rdx, 16(%rsp)
+ 	pushq	%rbp
+ 	.seh_pushreg %rbp
+-	subq	$32, %rsp
+-	.seh_stackalloc 32
+-	leaq	80(%rdx), %rbp
++	pushq	%rsi
++	.seh_pushreg %rsi
++	subq	$40, %rsp
++	.seh_stackalloc 40
++	leaq	32(%rdx), %rbp
  	.seh_endprologue
 -	movq	-8(%rbp), %rcx
--	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
--	leaq	.LBB0_4(%rip), %rax
+-	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
+-	movq	%rax, -40(%rbp)
+-	xorl	%eax, %eax
+-	movq	%rax, -32(%rbp)
+-	leaq	.LBB0_5(%rip), %rax
 -	addq	$32, %rsp
 -	popq	%rbp
 -	retq
@@ -36,46 +98,49 @@
 -	.long	($cppxdata$notable_function)@IMGREL
 -	.section	.text,"xr",one_only,notable_function
 -	.seh_endproc
--	.def	"?catch$3@?0?notable_function@4HA";
+-	.def	"?catch$10@?0?notable_function@4HA";
 -	.scl	3;
 -	.type	32;
 -	.endef
 -	.p2align	4, 0x90
--"?catch$3@?0?notable_function@4HA":
--.seh_proc "?catch$3@?0?notable_function@4HA"
+-"?catch$10@?0?notable_function@4HA":
+-.seh_proc "?catch$10@?0?notable_function@4HA"
 -	.seh_handler __CxxFrameHandler3, @unwind, @except
--.LBB0_3:
+-.LBB0_10:
 -	movq	%rdx, 16(%rsp)
 -	pushq	%rbp
 -	.seh_pushreg %rbp
 -	subq	$32, %rsp
 -	.seh_stackalloc 32
--	leaq	48(%rdx), %rbp
+-	leaq	80(%rdx), %rbp
 -	.seh_endprologue
+-	xorl	%eax, %eax
+-	movq	%rax, -32(%rbp)
 -	xorl	%ecx, %ecx
--	callq	_ZN3std9panicking3try7cleanup17h9c87e793fb4cdaabE
--	leaq	.LBB0_4(%rip), %rax
+-	callq	_ZN3std9panicking3try7cleanup17h09cddad23dc8025dE
+-	movq	%rax, -40(%rbp)
+-	leaq	.LBB0_5(%rip), %rax
 -	addq	$32, %rsp
 -	popq	%rbp
 -	retq
-+	callq	_ZN4core9panicking15panic_no_unwind17hedc8ed309bafae40E
++	callq	_ZN4core9panicking15panic_no_unwind17h49e55cbf27726099E
 +	ud2
  .Lfunc_end0:
  	.seh_handlerdata
  	.long	($cppxdata$notable_function)@IMGREL
-@@ -103,9 +67,9 @@
+@@ -129,9 +87,9 @@
  	.long	($stateUnwindMap$notable_function)@IMGREL
  	.long	1
  	.long	($tryMap$notable_function)@IMGREL
 -	.long	5
 +	.long	4
  	.long	($ip2state$notable_function)@IMGREL
--	.long	32
-+	.long	40
+-	.long	64
++	.long	32
  	.long	0
  	.long	1
  $stateUnwindMap$notable_function:
-@@ -117,18 +81,13 @@
+@@ -143,19 +101,14 @@
  	.long	0
  	.long	0
  	.long	1
@@ -85,23 +150,28 @@
  $handlerMap$0$notable_function:
 -	.long	8
 -	.long	__rust_panic_type_info@IMGREL
--	.long	40
--	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
+-	.long	72
+-	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
 -	.long	56
  	.long	64
  	.long	0
  	.long	0
--	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
-+	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
- 	.long	56
+-	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
+-	.long	56
++	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
++	.long	72
  $ip2state$notable_function:
  	.long	.Lfunc_begin0@IMGREL
-@@ -139,15 +98,4 @@
  	.long	-1
- 	.long	"?catch$2@?0?notable_function@4HA"@IMGREL
- 	.long	1
--	.long	"?catch$3@?0?notable_function@4HA"@IMGREL
+@@ -163,17 +116,6 @@
+ 	.long	0
+ 	.long	.Ltmp1@IMGREL+1
+ 	.long	-1
+-	.long	"?catch$9@?0?notable_function@4HA"@IMGREL
 -	.long	1
+-	.long	"?catch$10@?0?notable_function@4HA"@IMGREL
++	.long	"?catch$7@?0?notable_function@4HA"@IMGREL
+ 	.long	1
  	.section	.text,"xr",one_only,notable_function
 -
 -	.section	.data,"dw",discard,__rust_panic_type_info
