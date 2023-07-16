 llvm
%X = type { i8, i8 }

declare void @foo()

define void @f(%X* nocapture) {
  %2 = getelementptr inbounds %X, %X* %0, i64 0, i32 0
  store i8 1, i8* %2, align 1
  tail call void @foo()
  store i8 0, i8* %2, align 1
  ret void
}
