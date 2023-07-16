
define dso_local i64 @sub(i8* nocapture readonly %0) local_unnamed_addr #0 {
  %2 = load i8, i8* %0, align 1, !tbaa !3
  %3 = add i8 %2, -10
  %4 = sext i8 %3 to i64
  ret i64 %4
}
