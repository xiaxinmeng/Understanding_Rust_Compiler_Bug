
define hidden void @_ZN7example6Vector8to_array17h534c3671215fa5f8E([8 x i32]* noalias nocapture sret([8 x i32]) dereferenceable(32) %0, <8 x i32> %self) unnamed_addr #0 !dbg !5 {
  %1 = bitcast <8 x i32> %self to [8 x i32], !dbg !10
  store [8 x i32] %1, [8 x i32]* %0, align 4, !dbg !10
  ret void, !dbg !11
}
