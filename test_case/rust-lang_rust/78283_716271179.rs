diff
@@ -241,12 +241,8 @@
     %scevgep2 = getelementptr i8, i8* %color.sroa.0.164.in.3, i64 3
     store i8 -128, i8* %scevgep2, align 1
     %_7.i.i.i.i.3 = add i64 %color.sroa.13.163.3, -4
-    %.not.3 = icmp eq i8 %24, 0
     %scevgep3 = getelementptr i8, i8* %color.sroa.0.164.in.3, i64 4
-    br i1 %.not.3, label %bb4.loopexit.3, label %bb2.i.3
-  
-  bb4.loopexit.3:                                   ; preds = %bb19.3
-    ret void
+    br label %bb2.i.3
   }
