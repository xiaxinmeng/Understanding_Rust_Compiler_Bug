 armasm
 00000000 <core::panicking::panic>:
+        b580            push    {r7, lr}
+        466f            mov     r7, sp
         b088            sub     sp, #32
         4694            mov     ip, r2
         f240 0200       movw    r2, #0
