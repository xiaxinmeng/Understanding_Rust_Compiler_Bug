diff
-                       mov     rsi, qword ptr [r13 + 8]
-                       mov     rdi, qword ptr [r13 + 16]
-                       cmp     rdi, rsi
+                       mov     r15, qword ptr [rbp + 8]
+                       mov     rdi, qword ptr [rbp + 16]
+                       mov     rsi, rdi
+                       sub     rsi, r15
