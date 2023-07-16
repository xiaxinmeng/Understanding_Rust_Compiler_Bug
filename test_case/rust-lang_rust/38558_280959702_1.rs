llvm
; Function Attrs: nounwind uwtable
define void @f3(i32* nonnull, i32) unnamed_addr #0 {
entry-block:
  %2 = getelementptr inbounds i32, i32* %0, i32 %1
  %3 = icmp eq i32 %1, 0
  br i1 %3, label %bb4, label %bb5.preheader

bb5.preheader:                                    ; preds = %entry-block
  %4 = shl i32 %1, 2
  %5 = add i32 %4, -4
  %6 = lshr exact i32 %5, 2
  %7 = add nuw nsw i32 %6, 1
  %min.iters.check = icmp ult i32 %7, 4
  br i1 %min.iters.check, label %bb5.preheader19, label %min.iters.checked

bb5.preheader19:                                  ; preds = %middle.block, %min.iters.checked, %bb5.preheader
  %iter.sroa.0.0.in12.ph = phi i32* [ %0, %min.iters.checked ], [ %0, %bb5.preheader ], [ %ind.end, %middle.block ]
  br label %bb5

min.iters.checked:                                ; preds = %bb5.preheader
  %n.vec = and i32 %7, 2147483644
  %cmp.zero = icmp eq i32 %n.vec, 0
  %ind.end = getelementptr i32, i32* %0, i32 %n.vec
  br i1 %cmp.zero, label %bb5.preheader19, label %vector.body.preheader

vector.body.preheader:                            ; preds = %min.iters.checked
  br label %vector.body

vector.body:                                      ; preds = %vector.body.preheader, %vector.body
  %index = phi i32 [ %index.next, %vector.body ], [ 0, %vector.body.preheader ]
  %next.gep = getelementptr i32, i32* %0, i32 %index
  %8 = bitcast i32* %next.gep to <4 x i32>*
  %wide.load = load <4 x i32>, <4 x i32>* %8, align 4
  %9 = or <4 x i32> %wide.load, <i32 10, i32 10, i32 10, i32 10>
  %10 = bitcast i32* %next.gep to <4 x i32>*
  store <4 x i32> %9, <4 x i32>* %10, align 4
  %index.next = add i32 %index, 4
  %11 = icmp eq i32 %index.next, %n.vec
  br i1 %11, label %middle.block, label %vector.body, !llvm.loop !1

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i32 %7, %n.vec
  br i1 %cmp.n, label %bb4, label %bb5.preheader19

bb4.loopexit:                                     ; preds = %bb5
  br label %bb4

bb4:                                              ; preds = %bb4.loopexit, %middle.block, %entry-block
  ret void

bb5:                                              ; preds = %bb5.preheader19, %bb5
  %iter.sroa.0.0.in12 = phi i32* [ %12, %bb5 ], [ %iter.sroa.0.0.in12.ph, %bb5.preheader19 ]
  %12 = getelementptr inbounds i32, i32* %iter.sroa.0.0.in12, i32 1
  %13 = load i32, i32* %iter.sroa.0.0.in12, align 4
  %14 = or i32 %13, 10
  store i32 %14, i32* %iter.sroa.0.0.in12, align 4
  %15 = icmp eq i32* %12, %2
  br i1 %15, label %bb4.loopexit, label %bb5, !llvm.loop !4
}
