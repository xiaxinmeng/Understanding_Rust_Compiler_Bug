llvm-ir
; alloc::alloc::box_free
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @_ZN5alloc5alloc8box_free17h74d9b2058bbfc89bE(%"std::io::error::Custom"* %ptr) unnamed_addr #3 {
start:
  %0 = bitcast %"std::io::error::Custom"* %ptr to i8*
  tail call void @__rust_dealloc(i8* %0, i64 24, i64 8) #9
  ret void
} 

; alloc::alloc::box_free
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @_ZN5alloc5alloc8box_free17h95ac39da6ae7db02E({}* %ptr.0, {}* noalias nocapture nonnull readonly %ptr.1) unnamed_addr #3 {
start: 
  %0 = bitcast {}* %ptr.1 to i64*
  %1 = getelementptr inbounds i64, i64* %0, i64 1
  %2 = load i64, i64* %1, align 8, !invariant.load !6
  %3 = icmp eq i64 %2, 0
  br i1 %3, label %bb6, label %bb3
  
bb3:                                              ; preds = %start
  %4 = getelementptr inbounds i64, i64* %0, i64 2
  %5 = load i64, i64* %4, align 8, !invariant.load !6
  %6 = bitcast {}* %ptr.0 to i8*
  tail call void @__rust_dealloc(i8* %6, i64 %2, i64 %5) #9
  br label %bb6
  
bb6:                                              ; preds = %start, %bb3
  ret void
} 
