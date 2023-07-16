diff
--- a	2021-04-29 09:08:20.057255690 +0200
+++ b	2021-04-29 09:08:21.221282758 +0200
@@ -1,47 +1,47 @@
 <positdiv32>:
 	                   b8 00 00 00 80       	mov    $0x80000000,%eax
 	                   81 ff 00 00 00 80    	cmp    $0x80000000,%edi
 	            /----- 74 10                	je     <positdiv32+0x1d>
 	            |      89 f1                	mov    %esi,%ecx
 	            |      81 c9 00 00 00 80    	or     $0x80000000,%ecx
 	            |      81 f9 00 00 00 80    	cmp    $0x80000000,%ecx
 	            |  /-- 75 01                	jne    <positdiv32+0x1e>
 	            \--|-> c3                   	ret    
 	               \-> 85 ff                	test   %edi,%edi
 	      /----------- 74 4e                	je     <positdiv32+0x70>
 	      |            55                   	push   %rbp
 	      |            41 57                	push   %r15
 	      |            41 56                	push   %r14
 	      |            53                   	push   %rbx
-	      |            50                   	push   %rax
+	      |            48 83 ec 08          	sub    $0x8,%rsp
 	      |            41 0f 98 c7          	sets   %r15b
 	      |            85 f6                	test   %esi,%esi
 	      |            41 0f 98 c6          	sets   %r14b
 	      |            89 fa                	mov    %edi,%edx
 	      |            f7 da                	neg    %edx
 	      |            0f 4c d7             	cmovl  %edi,%edx
 	      |            89 f0                	mov    %esi,%eax
 	      |            f7 d8                	neg    %eax
 	      |            0f 4c c6             	cmovl  %esi,%eax
 	      |            8d 0c 95 00 00 00 00 	lea    0x0(,%rdx,4),%ecx
 	      |            f7 c2 00 00 00 40    	test   $0x40000000,%edx
-	      |  /-------- 0f 85 1f 00 00 00    	jne    <positdiv32+0x73>
+	      |  /-------- 75 20                	jne    <positdiv32+0x73>
 	      |  |         40 b5 ff             	mov    $0xff,%bpl
 	      |  |         85 c9                	test   %ecx,%ecx
-	      |  |  /----- 0f 88 09 00 00 00    	js     <positdiv32+0x68>
-	      |  |  |      90                   	nop
+	      |  |  /----- 0f 88 0a 00 00 00    	js     <positdiv32+0x68>
+	      |  |  |      66 90                	xchg   %ax,%ax
 	      |  |  |  /-> 40 80 c5 ff          	add    $0xff,%bpl
 	      |  |  |  |   01 c9                	add    %ecx,%ecx
 	      |  |  |  \-- 79 f8                	jns    <positdiv32+0x60>
 	      |  |  \----> 81 e1 fe ff ff 7f    	and    $0x7ffffffe,%ecx
 	      |  |  /----- eb 18                	jmp    <positdiv32+0x88>
 	      \--|--|----> 31 c0                	xor    %eax,%eax
 	         |  |      c3                   	ret    
 	         \--|----> 31 ed                	xor    %ebp,%ebp
 	            |      85 c9                	test   %ecx,%ecx
 	            +----- 0f 89 0b 00 00 00    	jns    <positdiv32+0x88>
 	            |      0f 1f 00             	nopl   (%rax)
 	            |  /-> 40 80 c5 01          	add    $0x1,%bpl
 	            |  |   01 c9                	add    %ecx,%ecx
 	            |  \-- 78 f8                	js     <positdiv32+0x80>
 	            \----> 89 cb                	mov    %ecx,%ebx
