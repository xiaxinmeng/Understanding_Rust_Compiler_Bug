llvm-ir
; alloc::alloc::box_free
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @_ZN5alloc5alloc8box_free17hdda4f80f00ae9ce2E(i8* nonnull %ptr.0, i8* noalias nonnull readonly %ptr.1) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds i8, i8* %ptr.1, i64 8
  %1 = bitcast i8* %0 to i64*
  %2 = load i64, i64* %1, align 8, !invariant.load !6
  %3 = icmp eq i64 %2, 0
  br i1 %3, label %bb7, label %bb4

bb4:                                              ; preds = %start
  %4 = getelementptr inbounds i8, i8* %ptr.1, i64 16
  %5 = bitcast i8* %4 to i64*
  %6 = load i64, i64* %5, align 8, !invariant.load !6
  tail call void @__rust_dealloc(i8* nonnull %ptr.0, i64 %2, i64 %6) #9
  br label %bb7

bb7:                                              ; preds = %start, %bb4
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @_ZN5alloc5alloc8box_free17hf06ac9a184e5da38E(i64* nonnull %ptr) unnamed_addr #3 {
start:
  %0 = bitcast i64* %ptr to i8*
  tail call void @__rust_dealloc(i8* nonnull %0, i64 24, i64 8) #9
  ret void
}
