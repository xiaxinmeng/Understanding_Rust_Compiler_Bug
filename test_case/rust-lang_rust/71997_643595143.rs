
define void @test([0 x i8]* nocapture nonnull align 1 %dest.0, i64 %dest.1, [0 x i8]* noalias nocapture nonnull readonly align 1 %src.0, i64 %src.1) {
start:
  %i = icmp ugt i64 %dest.1, %src.1
  %umin = select i1 %i, i64 %src.1, i64 %dest.1
  %i1 = icmp eq i64 %umin, 0
  br i1 %i1, label %bb7, label %bb9.preheader

bb9.preheader:                                    ; preds = %start
  br label %bb9

bb7.loopexit:                                     ; preds = %bb11
  br label %bb7

bb7:                                              ; preds = %bb7.loopexit, %start
  ret void

bb9:                                              ; preds = %bb11, %bb9.preheader
  %iv = phi i64 [ %iv.inc, %bb11 ], [ 0, %bb9.preheader ]
  %iv.inc = add nuw i64 %iv, 1
  %_23 = icmp ult i64 %iv, %src.1
  br i1 %_23, label %bb10, label %panic

bb10:                                             ; preds = %bb9
  %_26 = icmp ult i64 %iv, %dest.1
  br i1 %_26, label %bb11, label %panic1

bb11:                                             ; preds = %bb10
  %i3 = getelementptr inbounds [0 x i8], [0 x i8]* %src.0, i64 0, i64 %iv
  %_20 = load i8, i8* %i3, align 1
  %i4 = getelementptr inbounds [0 x i8], [0 x i8]* %dest.0, i64 0, i64 %iv
  store i8 %_20, i8* %i4, align 1
  %i5 = icmp ult i64 %iv.inc, %umin
  br i1 %i5, label %bb9, label %bb7.loopexit

panic:                                            ; preds = %bb9
  %iter.sroa.0.015.lcssa = phi i64 [ %iv, %bb9 ]
  tail call void @abort(i64 %iter.sroa.0.015.lcssa)
  unreachable

panic1:                                           ; preds = %bb10
  %iter.sroa.0.015.lcssa16 = phi i64 [ %iv, %bb10 ]
  tail call void @abort(i64 %iter.sroa.0.015.lcssa16)
  unreachable
}

declare void @abort(i64)
