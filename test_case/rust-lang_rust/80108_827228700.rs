
define void @_ZN7example6Vector8to_array17h38598b25c9720edeE([8 x i32]* noalias nocapture sret([8 x i32]) dereferenceable(32) %0, <8 x i32>* noalias nocapture dereferenceable(32) %self) unnamed_addr #0 !dbg !6 {
start:
  %1 = bitcast <8 x i32>* %self to [8 x i32]*, !dbg !11
  %2 = bitcast [8 x i32]* %0 to i8*, !dbg !11
  %3 = bitcast [8 x i32]* %1 to i8*, !dbg !11
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %2, i8* align 32 %3, i64 32, i1 false), !dbg !11
  ret void, !dbg !12
}
