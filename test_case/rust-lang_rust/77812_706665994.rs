llvm
define void @_ZN7example4test17h9c70bf34953e7145E() unnamed_addr #0 !dbg !6 {
start:
  %_2.i = alloca %"std::fmt::Arguments", align 8
  %0 = load i8, i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @GLOBAL, i64 0, i32 0, i64 0), align 1, !dbg !10, !range !11
  %_10.i.i.not = icmp eq i8 %0, 0, !dbg !12
  br i1 %_10.i.i.not, label %bb8, label %bb3, !dbg !22

bb3:                                              ; preds = %start
  %_6 = zext i8 %0 to i64, !dbg !23
  switch i64 %_6, label %bb5 [
    i64 0, label %bb4
    i64 1, label %bb6
    i64 2, label %bb7
  ], !dbg !23
