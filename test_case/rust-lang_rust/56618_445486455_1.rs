diff
--- ir_before.txt       2018-12-08 21:02:00.903516800 +0100
+++ ir_after.txt        2018-12-08 21:00:36.770815800 +0100
@@ -1,4 +1,4 @@
-# *** IR Dump Before X86 Avoid Store Forwarding Blocks ***:
+# *** IR Dump After X86 Avoid Store Forwarding Blocks ***:
 # Machine code for function _ZN126_$LT$tiberius..SqlConnection$LT$alloc..boxed..Box$LT$$LP$dyn$u20$tiberius..BoxableIo$u20$$u2b$$u20$$u27$static$RP$$GT$$GT$$GT$7connect17h74d4893aded596c4E: IsSSA, TracksLiveness
 Frame Objects:
   fi#0: size=1, align=1, at location [SP+8]
@@ -2054,10 +2054,20 @@
 ; predecessors: %bb.146
   successors: %bb.148(0x7ffff800), %bb.336(0x00000800); %bb.148(100.00%), %bb.336(0.00%)

-  %961:vr128 = MOVUPSrm %4:gr64, 1, $noreg, -4, $noreg :: (dereferenceable load 16 from %ir.1165, align 4)
+  %1272:gr16 = MOV16rm %4:gr64, 1, $noreg, -4, $noreg :: (dereferenceable load 2 from %ir.1165, align 4)
+  %1273:gr8 = MOV8rm %4:gr64, 1, $noreg, -2, $noreg :: (dereferenceable load 1 from %ir.1165 + 2, align 4)
+  %1274:gr32 = MOV32rm %4:gr64, 1, $noreg, 1, $noreg :: (dereferenceable load 4 from %ir.1165 + 3)
+  %1275:gr16 = MOV16rm %4:gr64, 1, $noreg, 5, $noreg :: (dereferenceable load 2 from %ir.1165 + 7, align 4)
+  %1276:gr32 = MOV32rm %4:gr64, 1, $noreg, 7, $noreg :: (dereferenceable load 4 from %ir.1165 + 9)
+  %1277:gr8 = MOV8rm %4:gr64, 1, $noreg, 11, $noreg :: (dereferenceable load 1 from %ir.1165 + 13, align 4)
   %962:vr128 = MOVUPSrm %4:gr64, 1, $noreg, 12, $noreg :: (dereferenceable load 16 from %ir.1165 + 16, align 4)
   MOVAPSmr %stack.22, 1, $noreg, 16, $noreg, killed %962:vr128 :: (store 16 into %ir.1166 + 16)
-  MOVAPSmr %stack.22, 1, $noreg, 0, $noreg, killed %961:vr128 :: (store 16 into %ir.1166)
+  MOV16mr %stack.22, 1, $noreg, 0, $noreg, killed %1272:gr16 :: (store 2 into %ir.1166, align 16)
+  MOV8mr %stack.22, 1, $noreg, 2, $noreg, killed %1273:gr8 :: (store 1 into %ir.1166 + 2, align 16)
+  MOV32mr %stack.22, 1, $noreg, 5, $noreg, killed %1274:gr32 :: (store 4 into %ir.1166 + 3, align 16)
+  MOV16mr %stack.22, 1, $noreg, 9, $noreg, killed %1275:gr16 :: (store 2 into %ir.1166 + 7, align 16)
+  MOV32mr %stack.22, 1, $noreg, 11, $noreg, killed %1276:gr32 :: (store 4 into %ir.1166 + 9, align 16)
+  MOV8mr %stack.22, 1, $noreg, 15, $noreg, killed %1277:gr8 :: (store 1 into %ir.1166 + 13, align 16)
   EH_LABEL <mcsymbol >
   ADJCALLSTACKDOWN64 32, 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp
   $rcx = COPY %673:gr64
@@ -2406,10 +2416,20 @@
 ; predecessors: %bb.171
   successors: %bb.173(0x7ffff800), %bb.337(0x00000800); %bb.173(100.00%), %bb.337(0.00%)

-  %882:vr128 = MOVUPSrm %4:gr64, 1, $noreg, -4, $noreg :: (dereferenceable load 16 from %ir.1363, align 4)
+  %1278:gr16 = MOV16rm %4:gr64, 1, $noreg, -4, $noreg :: (dereferenceable load 2 from %ir.1363, align 4)
+  %1279:gr8 = MOV8rm %4:gr64, 1, $noreg, -2, $noreg :: (dereferenceable load 1 from %ir.1363 + 2, align 4)
+  %1280:gr32 = MOV32rm %4:gr64, 1, $noreg, 1, $noreg :: (dereferenceable load 4 from %ir.1363 + 3)
+  %1281:gr16 = MOV16rm %4:gr64, 1, $noreg, 5, $noreg :: (dereferenceable load 2 from %ir.1363 + 7, align 4)
+  %1282:gr32 = MOV32rm %4:gr64, 1, $noreg, 7, $noreg :: (dereferenceable load 4 from %ir.1363 + 9)
+  %1283:gr8 = MOV8rm %4:gr64, 1, $noreg, 11, $noreg :: (dereferenceable load 1 from %ir.1363 + 13, align 4)
   %883:vr128 = MOVUPSrm %4:gr64, 1, $noreg, 12, $noreg :: (dereferenceable load 16 from %ir.1363 + 16, align 4)
   MOVAPSmr %stack.22, 1, $noreg, 16, $noreg, killed %883:vr128 :: (store 16 into %ir.1364 + 16)
-  MOVAPSmr %stack.22, 1, $noreg, 0, $noreg, killed %882:vr128 :: (store 16 into %ir.1364)
+  MOV16mr %stack.22, 1, $noreg, 0, $noreg, killed %1278:gr16 :: (store 2 into %ir.1364, align 16)
+  MOV8mr %stack.22, 1, $noreg, 2, $noreg, killed %1279:gr8 :: (store 1 into %ir.1364 + 2, align 16)
+  MOV32mr %stack.22, 1, $noreg, 5, $noreg, killed %1280:gr32 :: (store 4 into %ir.1364 + 3, align 16)
+  MOV16mr %stack.22, 1, $noreg, 9, $noreg, killed %1281:gr16 :: (store 2 into %ir.1364 + 7, align 16)
+  MOV32mr %stack.22, 1, $noreg, 11, $noreg, killed %1282:gr32 :: (store 4 into %ir.1364 + 9, align 16)
+  MOV8mr %stack.22, 1, $noreg, 15, $noreg, killed %1283:gr8 :: (store 1 into %ir.1364 + 13, align 16)
   EH_LABEL <mcsymbol >
   ADJCALLSTACKDOWN64 32, 0, 0, implicit-def dead $rsp, implicit-def dead $eflags, implicit-def dead $ssp, implicit $rsp, implicit $ssp
   $rcx = COPY %673:gr64
@@ -3832,11 +3852,13 @@
   %1093:gr32 = MOV32rm %stack.63, 1, $noreg, 0, $noreg :: (dereferenceable load 4 from %ir.2004)
   MOV32mr %stack.83, 1, $noreg, 0, $noreg, killed %1093:gr32 :: (store 4 into %ir.2006)
   %1094:vr128 = MOVAPSrm %stack.30, 1, $noreg, 0, $noreg :: (dereferenceable load 16 from %ir.2003)
-  %1095:vr128 = MOVAPSrm %stack.30, 1, $noreg, 16, $noreg :: (dereferenceable load 16 from %ir.2003 + 16)
+  %1284:gr64 = MOV64rm %stack.30, 1, $noreg, 16, $noreg :: (dereferenceable load 8 from %ir.2003 + 16, align 16)
+  %1285:gr64 = MOV64rm %stack.30, 1, $noreg, 24, $noreg :: (dereferenceable load 8 from %ir.2003 + 24, align 16)
   %1096:vr128 = MOVAPSrm %stack.30, 1, $noreg, 32, $noreg :: (dereferenceable load 16 from %ir.2003 + 32)
   %1097:vr128 = MOVAPSrm %stack.30, 1, $noreg, 48, $noreg :: (dereferenceable load 16 from %ir.2003 + 48)
   MOVAPSmr %stack.22, 1, $noreg, 0, $noreg, killed %1094:vr128 :: (store 16 into %ir.2005)
-  MOVAPSmr %stack.22, 1, $noreg, 16, $noreg, killed %1095:vr128 :: (store 16 into %ir.2005 + 16)
+  MOV64mr %stack.22, 1, $noreg, 16, $noreg, killed %1284:gr64 :: (store 8 into %ir.2005 + 16, align 16)
+  MOV64mr %stack.22, 1, $noreg, 24, $noreg, killed %1285:gr64 :: (store 8 into %ir.2005 + 24, align 16)
   MOVAPSmr %stack.22, 1, $noreg, 32, $noreg, killed %1096:vr128 :: (store 16 into %ir.2005 + 32)
   MOVAPSmr %stack.22, 1, $noreg, 48, $noreg, killed %1097:vr128 :: (store 16 into %ir.2005 + 48)
   %1098:vr128 = MOVAPSrm %stack.30, 1, $noreg, 64, $noreg :: (dereferenceable load 16 from %ir.2003 + 64)
