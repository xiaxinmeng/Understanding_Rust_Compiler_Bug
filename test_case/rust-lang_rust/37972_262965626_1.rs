llvm
; Function Attrs: norecurse nounwind readonly uwtable
define i64 @_Z9find_zeroRSt6vectorIcSaIcEE(%"class.std::vector"* nocapture readonly dereferenceable(24) %v) #0 {
  %1 = getelementptr inbounds %"class.std::vector", %"class.std::vector"* %v, i64 0, i32 0, i32 0, i32 0
  %2 = load i8*, i8** %1, align 8, !tbaa !1
  %3 = getelementptr inbounds %"class.std::vector", %"class.std::vector"* %v, i64 0, i32 0, i32 0, i32 1
  %4 = load i8*, i8** %3, align 8, !tbaa !1
  %5 = icmp eq i8* %2, %4
  %6 = ptrtoint i8* %2 to i64
  br i1 %5, label %._crit_edge, label %.lr.ph.preheader

.lr.ph.preheader:                                 ; preds = %0
  br label %.lr.ph

.lr.ph:                                           ; preds = %.lr.ph.preheader, %9
  %it.sroa.0.02 = phi i8* [ %10, %9 ], [ %2, %.lr.ph.preheader ]
  %7 = load i8, i8* %it.sroa.0.02, align 1, !tbaa !5
  %8 = icmp eq i8 %7, 0
  br i1 %8, label %._crit_edge.loopexit, label %9

; <label>:9                                       ; preds = %.lr.ph
  %10 = getelementptr inbounds i8, i8* %it.sroa.0.02, i64 1
  %11 = icmp eq i8* %10, %4
  br i1 %11, label %._crit_edge.loopexit, label %.lr.ph

._crit_edge.loopexit:                             ; preds = %9, %.lr.ph
  %it.sroa.0.0.lcssa.ph = phi i8* [ %4, %9 ], [ %it.sroa.0.02, %.lr.ph ]
  br label %._crit_edge

._crit_edge:                                      ; preds = %._crit_edge.loopexit, %0
  %it.sroa.0.0.lcssa = phi i8* [ %2, %0 ], [ %it.sroa.0.0.lcssa.ph, %._crit_edge.loopexit ]
  %12 = ptrtoint i8* %it.sroa.0.0.lcssa to i64
  %13 = sub i64 %12, %6
  ret i64 %13
}
