diff
@@ -79,28 +79,31 @@
 	.type	_ZN1a5Error8strerror17h116d2107cc85b4f7E,@function # -- Begin function _ZN1a5Error8strerror17h116d2107cc85b4f7E
 _ZN1a5Error8strerror17h116d2107cc85b4f7E: # @_ZN1a5Error8strerror17h116d2107cc85b4f7E
 # %bb.0:                                # %start
 	pushl	%esi
-	movzbl	8(%esp), %esi
 	calll	.L4$pb
 .L4$pb:
 	popl	%ecx
-	movl	$8, %edx
 .Ltmp3:
 	addl	$_GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L4$pb), %ecx
+	pushl	$8
+	movzbl	8(%esp), %esi
+	popl	%edx
 	leal	.Lalloc12@GOTOFF(%ecx), %eax
 	movl	.LJTI4_0@GOTOFF(%ecx,%esi,4), %esi
 	addl	%ecx, %esi
 	jmpl	*%esi
 .LBB4_1:                                # %bb3
 	leal	.Lalloc15@GOTOFF(%ecx), %eax
 	jmp	.LBB4_4
 .LBB4_2:                                # %bb4
-	movl	$4, %edx
+	pushl	$4
+	popl	%edx
 	leal	.Lalloc14@GOTOFF(%ecx), %eax
 	jmp	.LBB4_4
 .LBB4_3:                                # %bb5
-	movl	$7, %edx
+	pushl	$7
+	popl	%edx
 	leal	.Lalloc13@GOTOFF(%ecx), %eax
 .LBB4_4:                                # %bb6
 	popl	%esi
 	retl
