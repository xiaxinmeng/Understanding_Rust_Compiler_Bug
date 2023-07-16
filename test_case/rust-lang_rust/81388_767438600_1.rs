llvm
%Foo = type { [0 x i8], %Bar, [3 x i8], i32, [0 x i32], i32, [0 x i32] }
%Bar = type { [0 x i8], i8, [0 x i8] }

; Function Attrs: norecurse nounwind readnone
define void @foo(%Foo %0) unnamed_addr #0 {
start:
  ret void
}
