
--- orig.s      2017-02-03 00:31:43.635590663 +0100
+++ fixed.s     2017-02-03 00:32:14.347723354 +0100
@@ -23,7 +23,16 @@
        st %i0, [%fp+-8]
 .Ltmp4:
        .loc    1 8 0                   ! <anon>:8:0
-       ld [%i0+1], %i0
+       ldub [%i0+3], %i1
+       ldub [%i0+4], %i2
+       ldub [%i0+1], %i3
+       ldub [%i0+2], %i0
+       sll %i1, 8, %i1
+       or %i1, %i2, %i1
+       sll %i3, 8, %i2
+       or %i2, %i0, %i0
+       sll %i0, 16, %i0
+       or %i0, %i1, %i0
        .loc    1 9 0                   ! <anon>:9:0
        ret
        restore
