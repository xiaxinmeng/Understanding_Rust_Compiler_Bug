llvm
start:
  %0 = icmp eq i64 %data.meta, 0
  br i1 %0, label %bb5, label %bb2.i.preheader

bb2.i.preheader:                                  ; preds = %start
  br label %bb2.i

bb2.i:                                            ; preds = %bb7, %bb2.i.preheader
  %iter.sroa.0.010 = phi i64 [ %1, %bb7 ], [ 0, %bb2.i.preheader ]
  %1 = add nuw i64 %iter.sroa.0.010, 1
  br label %bb7 ; MANUALLY CHANGED

bb5.loopexit:                                     ; preds = %bb7, %bb2.i
  br label %bb5

bb5:                                              ; preds = %bb5.loopexit, %start
  ret void

bb7:                                              ; preds = %bb2.i
  %2 = getelementptr inbounds i8, i8* %data.ptr, i64 %iter.sroa.0.010
  store i8 0, i8* %2, align 1
  %3 = icmp ult i64 %1, %data.meta
  br i1 %3, label %bb2.i, label %bb5.loopexit
}

; Function Attrs: nounwind readnone
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #0

attributes #0 = { nounwind readnone }
