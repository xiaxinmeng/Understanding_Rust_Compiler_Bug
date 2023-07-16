
❯ diff -u t-ios.ll t-sim.ll  | head -8
--- t-ios.ll	2021-02-12 11:10:21.000000000 +0100
+++ t-sim.ll	2021-02-12 11:10:03.000000000 +0100
@@ -1,7 +1,7 @@
 ; ModuleID = 't.c'
 source_filename = "t.c"
 target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
-target triple = "arm64-apple-ios7.0.0"
+target triple = "arm64-apple-ios7.0.0-simulator"
