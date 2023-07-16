llvm
define void @_ZN7example3foo17hd23b272186309284E() unnamed_addr #0 !dbg !5 {
  %x = alloca { [0 x i32], i32, [0 x i8], { [0 x i8] }, [0 x i8] }, align 4
  %0 = bitcast { [0 x i32], i32, [0 x i8], { [0 x i8] }, [0 x i8] }* %x to i32*, !dbg !8
  store i32 100, i32* %0, align 4, !dbg !8
  ret void, !dbg !10
}
