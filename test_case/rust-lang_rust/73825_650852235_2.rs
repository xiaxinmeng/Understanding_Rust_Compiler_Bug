llvm
@__const.foo.bases = private unnamed_addr constant [5 x i32] [i32 1, i32 3, i32 4, i32 5, i32 2], align 16

define dso_local i32 @foo(i64 %0) #0 !dbg !7 {
  %2 = alloca i64, align 8
  %3 = alloca [5 x i32], align 16
  store i64 %0, i64* %2, align 8
  call void @llvm.dbg.declare(metadata i64* %2, metadata !19, metadata !DIExpression()), !dbg !20
  call void @llvm.dbg.declare(metadata [5 x i32]* %3, metadata !21, metadata !DIExpression()), !dbg !25
  %4 = bitcast [5 x i32]* %3 to i8*, !dbg !25
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %4, i8* align 16 bitcast ([5 x i32]* @__const.foo.bases to i8*), i64 20, i1 false), !dbg !25
  %5 = load i64, i64* %2, align 8, !dbg !26
  %6 = urem i64 %5, 5, !dbg !27
  %7 = getelementptr inbounds [5 x i32], [5 x i32]* %3, i64 0, i64 %6, !dbg !28
  %8 = load i32, i32* %7, align 4, !dbg !28
  ret i32 %8, !dbg !29
}
