
; Function Attrs: uwtable
define void @_Z12map_on_elemsRKSt6vectorIiSaIiEERFviE(%"class.std::vector"* nocapture readonly dereferenceable(24), void (i32)* nocapture) local_unnamed_addr #0 {
  %3 = getelementptr inbounds %"class.std::vector", %"class.std::vector"* %0, i64 0, i32 0, i32 0, i32 1
  %4 = bitcast i32** %3 to i64*
  %5 = load i64, i64* %4, align 8, !tbaa !2
  %6 = bitcast %"class.std::vector"* %0 to i64*
  %7 = load i64, i64* %6, align 8, !tbaa !8
  %8 = icmp eq i64 %5, %7
  br i1 %8, label %10, label %9

; <label>:9: ; preds = %2
  br label %11

; <label>:10: ; preds = %11, %2
  ret void

; <label>:11: ; preds = %9, %11
  %12 = phi i64 [ %19, %11 ], [ %7, %9 ]
  %13 = phi i64 [ %17, %11 ], [ 0, %9 ]
  %14 = inttoptr i64 %12 to i32*
  %15 = getelementptr inbounds i32, i32* %14, i64 %13
  %16 = load i32, i32* %15, align 4, !tbaa !9
  tail call void %1(i32 %16)
  %17 = add i64 %13, 1
  %18 = load i64, i64* %4, align 8, !tbaa !2
  %19 = load i64, i64* %6, align 8, !tbaa !8
  %20 = sub i64 %18, %19
  %21 = ashr exact i64 %20, 2
  %22 = icmp ult i64 %17, %21
  br i1 %22, label %11, label %10
}
