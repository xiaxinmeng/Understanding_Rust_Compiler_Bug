
; Function Attrs: norecurse nounwind ssp uwtable
define void @foo(i32* nocapture, i32* nocapture, i32* nocapture readonly) #0 {
  %4 = load i32, i32* %2, align 4, !tbaa !2
  store i32 %4, i32* %0, align 4, !tbaa !2
  %5 = load i32, i32* %2, align 4, !tbaa !2
  store i32 %5, i32* %1, align 4, !tbaa !2
  ret void
}
