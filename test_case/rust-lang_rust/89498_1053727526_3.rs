llvm
; simple::t
; Function Attrs: nonlazybind uwtable
define internal void @_ZN6simple1t17h9b9464898cd4c360E({ i64, i32 }* noalias nocapture noundef sret({ i64, i32 }) dereferenceable(12) %0) unnamed_addr #0 {
start:
  %1 = bitcast { i64, i32 }* %0 to i64*
  store i64 1000, i64* %1, align 4
  %2 = getelementptr inbounds { i64, i32 }, { i64, i32 }* %0, i32 0, i32 1
  store i32 3000, i32* %2, align 4
  ret void
}
