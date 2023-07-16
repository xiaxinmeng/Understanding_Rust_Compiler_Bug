 diff
--- /home/nmatsakis/tmp/print-ok-beta   2016-09-15 12:22:36.604531321 -0400
+++ /home/nmatsakis/tmp/print-ref-beta  2016-09-15 12:23:04.984530867 -0400
@@ -10,6 +10,7 @@
     .cfi_def_cfa_offset 80
     cmpb    $1, %dil
     jne    .LBB0_1
+    andb    $3, %sil
     cmpb    $1, %sil
     je    .LBB0_7
     cmpb    $2, %sil
