llvm
define i32 @_ZN10playground3foo17ha3dfdbe3e3d033d9E(i64 %x) unnamed_addr #0 !dbg !6 {
start:
  %x.dbg.spill = alloca i64, align 8
  %base = alloca [5 x i32], align 4
  store i64 %x, i64* %x.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64* %x.dbg.spill, metadata !14, metadata !DIExpression()), !dbg !20
  call void @llvm.dbg.declare(metadata [5 x i32]* %base, metadata !15, metadata !DIExpression()), !dbg !21
  %0 = bitcast [5 x i32]* %base to i32*, !dbg !22
  store i32 1, i32* %0, align 4, !dbg !22
  %1 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i32 0, i32 1, !dbg !22
  store i32 3, i32* %1, align 4, !dbg !22
  %2 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i32 0, i32 2, !dbg !22
  store i32 4, i32* %2, align 4, !dbg !22
  %3 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i32 0, i32 3, !dbg !22
  store i32 5, i32* %3, align 4, !dbg !22
  %4 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i32 0, i32 4, !dbg !22
  store i32 2, i32* %4, align 4, !dbg !22
  %_3 = urem i64 %x, 5, !dbg !23
  %_6 = icmp ult i64 %_3, 5, !dbg !24
  %5 = call i1 @llvm.expect.i1(i1 %_6, i1 true), !dbg !24
  br i1 %5, label %bb1, label %panic, !dbg !24

bb1:                                              ; preds = %start
  %6 = getelementptr inbounds [5 x i32], [5 x i32]* %base, i64 0, i64 %_3, !dbg !24
  %7 = load i32, i32* %6, align 4, !dbg !24
  ret i32 %7, !dbg !25

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h4b3d0dcda831e378E(i64 %_3, i64 5, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc3 to %"core::panic::Location"*)), !dbg !24
  unreachable, !dbg !24
}
