llvm
; min::S::f
; Function Attrs: norecurse nounwind uwtable
define void @_ZN3min1S1f17h1ac21da989830415E(%S* nocapture dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds %S, %S* %self, i64 0, i32 3, i64 1
  store i8 0, i8* %0, align 8
  %1 = getelementptr inbounds %S, %S* %self, i64 0, i32 3, i64 2
  store i8 0, i8* %1, align 8
  ret void
}
