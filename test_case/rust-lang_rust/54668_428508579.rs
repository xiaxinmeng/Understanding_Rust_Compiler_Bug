diff
-define void @_ZN10playground29swap_nonoverlapping_bytes_old17h49ccafcb349957d5E(i8* nocapture %x, i8* nocapture %y, i64 %len) unnamed_addr #0 {
+define void @_ZN10playground31swap_nonoverlapping_bytes_fixed17h9e199c2b3be909ffE(i8* nocapture %x, i8* nocapture %y, i64 %len) unnamed_addr #0 {
 start:
-  %t1 = alloca %"swap_nonoverlapping_bytes_old::UnalignedBlock", align 8
+  %t1.sroa.0 = alloca [4 x i64], align 8
+  %t.sroa.0 = alloca [4 x i64], align 32
   %0 = icmp ult i64 %len, 32
-  br i1 %0, label %bb3, label %bb4
+  br i1 %0, label %bb3, label %bb4.lr.ph
+
+bb4.lr.ph:                                        ; preds = %start
+  %t.sroa.0.0.sroa_cast13 = bitcast [4 x i64]* %t.sroa.0 to i8*
+  br label %bb4
 
 bb3:                                              ; preds = %bb4, %start
   %i.0.lcssa = phi i64 [ 0, %start ], [ %2, %bb4 ]
   %1 = icmp ult i64 %i.0.lcssa, %len
-  br i1 %1, label %bb11, label %bb18
+  br i1 %1, label %bb12, label %bb20
 
-bb4:                                              ; preds = %start, %bb4
-  %2 = phi i64 [ %5, %bb4 ], [ 32, %start ]
-  %i.016 = phi i64 [ %2, %bb4 ], [ 0, %start ]
-  %3 = getelementptr inbounds i8, i8* %x, i64 %i.016
-  %4 = getelementptr inbounds i8, i8* %y, i64 %i.016
-  %t.0..sroa_cast = bitcast i8* %3 to <4 x i64>*
-  %t.0.copyload = load <4 x i64>, <4 x i64>* %t.0..sroa_cast, align 1
+bb4:                                              ; preds = %bb4.lr.ph, %bb4
+  %2 = phi i64 [ 32, %bb4.lr.ph ], [ %5, %bb4 ]
+  %i.022 = phi i64 [ 0, %bb4.lr.ph ], [ %2, %bb4 ]
+  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %t.sroa.0.0.sroa_cast13)
+  %3 = getelementptr inbounds i8, i8* %x, i64 %i.022
+  %4 = getelementptr inbounds i8, i8* %y, i64 %i.022
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 32 %t.sroa.0.0.sroa_cast13, i8* align 1 %3, i64 32, i1 false)
   tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 %4, i64 32, i1 false)
-  %t.0..sroa_cast10 = bitcast i8* %4 to <4 x i64>*
-  store <4 x i64> %t.0.copyload, <4 x i64>* %t.0..sroa_cast10, align 1
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* nonnull align 32 %t.sroa.0.0.sroa_cast13, i64 32, i1 false)
+  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %t.sroa.0.0.sroa_cast13)
   %5 = add i64 %2, 32
   %6 = icmp ugt i64 %5, %len
   br i1 %6, label %bb3, label %bb4
 
-bb11:                                             ; preds = %bb3
-  %t1.0.sroa_cast14 = bitcast %"swap_nonoverlapping_bytes_old::UnalignedBlock"* %t1 to i8*
-  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %t1.0.sroa_cast14)
+bb12:                                             ; preds = %bb3
+  %t1.sroa.0.0.sroa_cast20 = bitcast [4 x i64]* %t1.sroa.0 to i8*
+  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %t1.sroa.0.0.sroa_cast20)
   %7 = sub i64 %len, %i.0.lcssa
   %8 = getelementptr inbounds i8, i8* %x, i64 %i.0.lcssa
   %9 = getelementptr inbounds i8, i8* %y, i64 %i.0.lcssa
-  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %t1.0.sroa_cast14, i8* align 1 %8, i64 %7, i1 false)
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %t1.sroa.0.0.sroa_cast20, i8* align 1 %8, i64 %7, i1 false)
   tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %8, i8* align 1 %9, i64 %7, i1 false)
-  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %9, i8* nonnull align 8 %t1.0.sroa_cast14, i64 %7, i1 false)
-  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %t1.0.sroa_cast14)
-  br label %bb18
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %9, i8* nonnull align 8 %t1.sroa.0.0.sroa_cast20, i64 %7, i1 false)
+  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %t1.sroa.0.0.sroa_cast20)
+  br label %bb20
 
-bb18:                                             ; preds = %bb11, %bb3
+bb20:                                             ; preds = %bb12, %bb3
   ret void
 }
