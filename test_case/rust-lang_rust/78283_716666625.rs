diff
 --- main.ll	2020-10-26 12:25:05 -0400
+++ main-1.37-nuw.ll	2020-10-26 12:24:24 -0400
@@ -211,7 +211,7 @@
   %color.sroa.13.159.3 = phi i64 [ %53, %bb22.3 ], [ %42, %bb3.i.2 ]
   %iter1.sroa.0.058.3 = phi i8 [ %44, %bb22.3 ], [ 0, %bb3.i.2 ]
   %43 = icmp eq i8 %iter1.sroa.0.058.3, -1
-  %44 = add i8 %iter1.sroa.0.058.3, 1
+  %44 = add nuw i8 %iter1.sroa.0.058.3, 1
   %45 = icmp eq i64 %color.sroa.13.159.3, 0
   br i1 %45, label %panic, label %bb18.3, !prof !13
 