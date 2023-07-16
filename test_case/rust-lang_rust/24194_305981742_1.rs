
; Function Attrs: nounwind sspstrong uwtable
define void @wowzers(i32) local_unnamed_addr #0 {
  %2 = zext i32 %0 to i64
  %3 = tail call noalias i8* @malloc(i64 %2) #2
  %4 = icmp eq i8* %3, null
  br i1 %4, label %6, label %5
; <label>:5:                                      ; preds = %1
  tail call void @free(i8* nonnull %3) #2
  br label %6
; <label>:6:                                      ; preds = %1, %5
  ret void
}
