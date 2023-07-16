diff
--- before.s	2020-08-04 01:00:06.506321119 +0200
+++ after.s	2020-08-04 01:02:00.953935361 +0200
@@ -1,91 +1,60 @@
 	.text
 	.file	"panic.8h59gtm7-cgu.0"
-	.section	".text._ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E","ax",@progbits
-	.p2align	4, 0x90
-	.type	_ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E,@function
-_ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E:
-	.cfi_startproc
-	pushq	%rax
-	.cfi_def_cfa_offset 16
-	movq	(%rdi), %rcx
-	movq	8(%rdi), %rdx
-	movq	(%rcx), %rax
-	movq	8(%rcx), %rsi
-	movq	(%rdx), %rdx
-	movq	16(%rdi), %rcx
-	movq	(%rcx), %rcx
-	leaq	.L__unnamed_1(%rip), %r8
-	movq	%rax, %rdi
-	callq	*_ZN4core3str16slice_error_fail17he51ff928bfc853b6E@GOTPCREL(%rip)
-	ud2
-.Lfunc_end0:
-	.size	_ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E, .Lfunc_end0-_ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E
-	.cfi_endproc
-
 	.section	.text._ZN5panic6substr17h2e99bd59166da18bE,"ax",@progbits
 	.globl	_ZN5panic6substr17h2e99bd59166da18bE
 	.p2align	4, 0x90
 	.type	_ZN5panic6substr17h2e99bd59166da18bE,@function
 _ZN5panic6substr17h2e99bd59166da18bE:
 	.cfi_startproc
-	subq	$56, %rsp
-	.cfi_def_cfa_offset 64
+	pushq	%rax
+	.cfi_def_cfa_offset 16
 	movq	%rdi, %rax
-	movq	%rdi, 16(%rsp)
-	movq	%rsi, 24(%rsp)
-	movq	%rdx, (%rsp)
-	movq	%rcx, 8(%rsp)
 	movq	%rcx, %rdi
 	subq	%rdx, %rdi
-	jb	.LBB1_10
+	jb	.LBB0_10
 	testq	%rdx, %rdx
-	je	.LBB1_2
+	je	.LBB0_2
 	cmpq	%rdx, %rsi
-	je	.LBB1_2
-	jbe	.LBB1_10
+	je	.LBB0_2
+	jbe	.LBB0_10
 	cmpb	$-65, (%rax,%rdx)
-	jle	.LBB1_10
-.LBB1_2:
+	jle	.LBB0_10
+.LBB0_2:
 	testq	%rcx, %rcx
-	je	.LBB1_6
+	je	.LBB0_6
 	cmpq	%rcx, %rsi
-	je	.LBB1_6
-	jbe	.LBB1_10
+	je	.LBB0_6
+	jbe	.LBB0_10
 	cmpb	$-65, (%rax,%rcx)
-	jle	.LBB1_10
-.LBB1_6:
+	jle	.LBB0_10
+.LBB0_6:
 	addq	%rdx, %rax
 	movq	%rdi, %rdx
-	addq	$56, %rsp
+	popq	%rcx
 	.cfi_def_cfa_offset 8
 	retq
-.LBB1_10:
-	.cfi_def_cfa_offset 64
-	leaq	16(%rsp), %rax
-	movq	%rax, 32(%rsp)
-	movq	%rsp, %rax
-	movq	%rax, 40(%rsp)
-	leaq	8(%rsp), %rax
-	movq	%rax, 48(%rsp)
-	leaq	32(%rsp), %rdi
-	callq	_ZN4core3str6traits101_$LT$impl$u20$core..slice..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17h75d822db790d87d2E
+.LBB0_10:
+	.cfi_def_cfa_offset 16
+	leaq	.L__unnamed_1(%rip), %r8
+	movq	%rax, %rdi
+	callq	*_ZN4core3str16slice_error_fail17he51ff928bfc853b6E@GOTPCREL(%rip)
 	ud2
-.Lfunc_end1:
-	.size	_ZN5panic6substr17h2e99bd59166da18bE, .Lfunc_end1-_ZN5panic6substr17h2e99bd59166da18bE
+.Lfunc_end0:
+	.size	_ZN5panic6substr17h2e99bd59166da18bE, .Lfunc_end0-_ZN5panic6substr17h2e99bd59166da18bE
 	.cfi_endproc
 
 	.type	.L__unnamed_2,@object
 	.section	.rodata..L__unnamed_2,"a",@progbits
 .L__unnamed_2:
-	.ascii	"/home/tm/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs"
-	.size	.L__unnamed_2, 109
+	.ascii	"src/lib.rs"
+	.size	.L__unnamed_2, 10
 
 	.type	.L__unnamed_1,@object
 	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
 	.p2align	3
 .L__unnamed_1:
 	.quad	.L__unnamed_2
-	.asciz	"m\000\000\000\000\000\000\000\206\007\000\000/\000\000"
+	.asciz	"\n\000\000\000\000\000\000\000\002\000\000\000\006\000\000"
 	.size	.L__unnamed_1, 24
 
 	.section	".note.GNU-stack","",@progbits
