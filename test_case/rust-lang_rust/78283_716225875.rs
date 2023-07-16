diff
--- bad.s	2020-10-25 18:48:19.000000000 -0400
+++ good.s	2020-10-25 18:47:59.000000000 -0400
@@ -12,7 +12,9 @@
 	.endef
 	.p2align	4, 0x90                         # -- Begin function _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h39f1d54c4354cfb7E
 _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h39f1d54c4354cfb7E: # @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h39f1d54c4354cfb7E
+.Lfunc_begin0:
 .seh_proc _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h39f1d54c4354cfb7E
+	.seh_handler __CxxFramHandler3, @unwind, @except
 # %bb.0:                                # %start
 	subq	$40, %rsp
 	.seh_stackalloc 40
@@ -23,9 +25,26 @@
 	#NO_APP
 	addq	$40, %rsp
 	retq
+.Lfunc_end0:
 	.seh_handlerdata
 	.text
 	.seh_endproc
+	.section	.xdata,"dr"
+	.p2align	2
+GCC_except_table0:
+.Lexception0:
+	.byte	255                             # @LPStart Encoding = omit
+	.byte	255                             # @TType Encoding = omit
+	.byte	1                               # Call site Encoding = uleb128
+	.uleb128 .Lcst_end0-.Lcst_begin0
+.Lcst_begin0:
+	.uleb128 .Lfunc_begin0-.Lfunc_begin0    # >> Call Site 1 <<
+	.uleb128 .Lfunc_end0-.Lfunc_begin0      #   Call between .Lfunc_begin0 and .Lfunc_end0
+	.byte	0                               #     has no landing pad
+	.byte	0                               #   On action: cleanup
+.Lcst_end0:
+	.p2align	2
+	.text
                                         # -- End function
 	.def	 _ZN3std2rt10lang_start17h44748352664c46c2E;
 	.scl	2;
@@ -107,24 +126,30 @@
 	.endef
 	.p2align	4, 0x90                         # -- Begin function _ZN3out5inner17h353a9cc7a3c23810E
 _ZN3out5inner17h353a9cc7a3c23810E:      # @_ZN3out5inner17h353a9cc7a3c23810E
+.Lfunc_begin1:
 .seh_proc _ZN3out5inner17h353a9cc7a3c23810E
+	.seh_handler __CxxFramHandler3, @unwind, @except
 # %bb.0:                                # %start
 	pushq	%rsi
 	.seh_pushreg %rsi
+	pushq	%rdi
+	.seh_pushreg %rdi
+	pushq	%rbx
+	.seh_pushreg %rbx
 	subq	$32, %rsp
 	.seh_stackalloc 32
 	.seh_endprologue
-	movq	%rcx, %rsi
+	movq	%rcx, %rbx
 	movl	$4096, %ecx                     # imm = 0x1000
 	movl	$1, %edx
 	callq	__rust_alloc_zeroed
 	testq	%rax, %rax
-	je	.LBB5_17
+	je	.LBB5_18
 # %bb.1:                                # %_ZN5alloc3vec9from_elem17h74690dcc847adf77E.exit
-	movq	%rax, (%rsi)
-	movq	$4096, 8(%rsi)                  # imm = 0x1000
-	movq	$4096, 16(%rsi)                 # imm = 0x1000
-	movq	$-4096, %r8                     # imm = 0xF000
+	movq	%rax, (%rbx)
+	movq	$4096, 8(%rbx)                  # imm = 0x1000
+	movq	$4096, 16(%rbx)                 # imm = 0x1000
+	movq	$-4096, %rdx                    # imm = 0xF000
 	xorl	%ecx, %ecx
 	.p2align	4, 0x90
 .LBB5_2:                                # %bb2.i
@@ -137,7 +162,7 @@
 	movb	%cl, 6(%rax)
 	movb	$-128, 7(%rax)
 	addq	$8, %rax
-	addq	$8, %r8
+	addq	$8, %rdx
 	incb	%cl
 	jne	.LBB5_2
 # %bb.3:                                # %bb2.i.1.preheader
@@ -150,16 +175,16 @@
 	movb	%sil, 2(%rax,%rsi,4)
 	movw	$-32640, 3(%rax,%rsi,4)         # imm = 0x8080
 	movb	$-128, 5(%rax,%rsi,4)
-	leal	1(%rsi), %edx
-	movb	%dl, 6(%rax,%rsi,4)
+	leal	1(%rsi), %edi
+	movb	%dil, 6(%rax,%rsi,4)
 	movb	$-128, 7(%rax,%rsi,4)
 	addq	$-8, %rcx
 	addq	$2, %rsi
-	cmpb	$-1, %dl
+	cmpb	$-1, %dil
 	jne	.LBB5_4
 # %bb.5:                                # %bb2.i.2.preheader
 	subq	%rcx, %rax
-	subq	%r8, %rcx
+	subq	%rdx, %rcx
 	xorl	%edx, %edx
 	.p2align	4, 0x90
 .LBB5_6:                                # %bb2.i.2
@@ -177,32 +202,41 @@
 	jne	.LBB5_6
 # %bb.7:                                # %bb2.i.3.preheader
 	xorl	%edx, %edx
-	testq	%rcx, %rcx
-	je	.LBB5_9
 	.p2align	4, 0x90
-.LBB5_13:                               # %bb16.3
+.LBB5_8:                                # %bb2.i.3
                                         # =>This Inner Loop Header: Depth=1
+	testq	%rcx, %rcx
+	je	.LBB5_9
+# %bb.13:                               # %bb16.3
+                                        #   in Loop: Header=BB5_8 Depth=1
 	movb	$-128, (%rax)
 	cmpq	$1, %rcx
 	je	.LBB5_10
 # %bb.14:                               # %bb17.3
-                                        #   in Loop: Header=BB5_13 Depth=1
+                                        #   in Loop: Header=BB5_8 Depth=1
 	leal	1(%rdx), %esi
 	movb	$-128, 1(%rax)
 	cmpq	$3, %rcx
 	jb	.LBB5_11
 # %bb.15:                               # %bb18.3
-                                        #   in Loop: Header=BB5_13 Depth=1
+                                        #   in Loop: Header=BB5_8 Depth=1
 	movb	%dl, 2(%rax)
 	je	.LBB5_12
 # %bb.16:                               # %bb19.3
-                                        #   in Loop: Header=BB5_13 Depth=1
+                                        #   in Loop: Header=BB5_8 Depth=1
 	movb	$-128, 3(%rax)
 	addq	$-4, %rcx
 	addq	$4, %rax
 	movl	%esi, %edx
-	testq	%rcx, %rcx
-	jne	.LBB5_13
+	testb	%sil, %sil
+	jne	.LBB5_8
+# %bb.17:                               # %bb4.loopexit.3
+	movq	%rbx, %rax
+	addq	$32, %rsp
+	popq	%rbx
+	popq	%rdi
+	popq	%rsi
+	retq
 .LBB5_9:                                # %panic
 	leaq	.Lalloc20(%rip), %r8
 	xorl	%ecx, %ecx
@@ -223,14 +257,27 @@
 	movl	$3, %ecx
 	movl	$3, %edx
 	callq	_ZN4core9panicking18panic_bounds_check17h2d25ebb349b8ce6eE
-.LBB5_17:                               # %bb20.i.i.i.i.i
+.LBB5_18:                               # %bb20.i.i.i.i.i
 	movl	$4096, %ecx                     # imm = 0x1000
 	movl	$1, %edx
 	callq	_ZN5alloc5alloc18handle_alloc_error17h71c060cff7245371E
 	int3
+.Lfunc_end1:
 	.seh_handlerdata
 	.text
 	.seh_endproc
+	.section	.xdata,"dr"
+	.p2align	2
+GCC_except_table5:
+.Lexception1:
+	.byte	255                             # @LPStart Encoding = omit
+	.byte	255                             # @TType Encoding = omit
+	.byte	1                               # Call site Encoding = uleb128
+	.uleb128 .Lcst_end1-.Lcst_begin1
+.Lcst_begin1:
+.Lcst_end1:
+	.p2align	2
+	.text
                                         # -- End function
 	.def	 _ZN3out4main17ha1c9ae0fb8136a1fE;
 	.scl	3;
