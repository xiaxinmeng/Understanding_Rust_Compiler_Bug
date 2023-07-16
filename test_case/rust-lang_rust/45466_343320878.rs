llvm
; Function Attrs: uwtable
define void @_ZN7suspect7memzero17h76c06c0c84a550b5E(i8* nocapture nonnull %data.ptr, i64 %data.meta) {
start:
  %0 = icmp eq i64 %data.meta, 0
  br i1 %0, label %bb5, label %bb2.i.preheader

bb2.i.preheader:                                  ; preds = %start
  br label %bb2.i

bb2.i:                                            ; preds = %bb2.i.preheader, %bb7
  %iter.sroa.0.010 = phi i64 [ %3, %bb7 ], [ 0, %bb2.i.preheader ]
  %1 = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %iter.sroa.0.010, i64 1) #5
  %2 = extractvalue { i64, i1 } %1, 1
  br i1 %2, label %bb5.loopexit, label %bb7

bb5.loopexit:                                     ; preds = %bb7, %bb2.i
  br label %bb5

bb5:                                              ; preds = %bb5.loopexit, %start
  ret void

bb7:                                              ; preds = %bb2.i
  %3 = extractvalue { i64, i1 } %1, 0
  %4 = getelementptr inbounds i8, i8* %data.ptr, i64 %iter.sroa.0.010
  store i8 0, i8* %4, align 1
  %5 = icmp ult i64 %3, %data.meta
  br i1 %5, label %bb2.i, label %bb5.loopexit
}
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64)
