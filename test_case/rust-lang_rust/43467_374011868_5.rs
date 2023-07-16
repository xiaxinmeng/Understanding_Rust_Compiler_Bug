llvm
%"::c_void" = type {}

define void @call_malloc() unnamed_addr #0 !dbg !4 {
start:
  %0 = call %"::c_void"* @malloc(i64 1), !dbg !8
  br label %bb1, !dbg !8

bb1:                                              ; preds = %start
  ret void, !dbg !10
}

declare %"::c_void"* @malloc(i64) unnamed_addr #1
