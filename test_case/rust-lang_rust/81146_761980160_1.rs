llvm
; main::foo
; Function Attrs: noinline nonlazybind uwtable
define internal i128 @_ZN4main3foo17hb520db95bcf5df69E() unnamed_addr #2 {
start:
  %0 = alloca i128, align 8
  %self = alloca %"std::result::Result<(), Error>", align 8
  %1 = alloca %"std::result::Result<(), Error>", align 8
; call main::bar
  %2 = call i128 @_ZN4main3bar17h199b6044e637b435E()
  %3 = bitcast i128* %0 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %3)
  store i128 %2, i128* %0, align 8
  %4 = bitcast %"std::result::Result<(), Error>"* %self to i8*
  %5 = bitcast i128* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 16, i1 false)
  %6 = bitcast i128* %0 to i8*
  call void @llvm.lifetime.end.p0i8(i64 16, i8* %6)
  br label %bb4

bb1:                                              ; preds = %bb4
; call main::println
  call void @_ZN4main7println17h4b182729f4c1c974E()
  br label %bb3

bb2:                                              ; preds = %bb4
  %7 = bitcast %"std::result::Result<(), Error>"* %1 to %"std::result::Result<(), Error>::Err"*
  %8 = bitcast %"std::result::Result<(), Error>::Err"* %7 to %Error*
  %9 = bitcast %"std::result::Result<(), Error>"* %self to %"std::result::Result<(), Error>::Err"*
  %10 = bitcast %"std::result::Result<(), Error>::Err"* %9 to %Error*
  %11 = bitcast %Error* %8 to i8*
  %12 = bitcast %Error* %10 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %11, i8* align 8 %12, i64 16, i1 false)
  %13 = bitcast %"std::result::Result<(), Error>"* %1 to i128*
  %14 = load i128, i128* %13, align 8
  ret i128 %14

bb3:                                              ; preds = %bb1
  %15 = bitcast %"std::result::Result<(), Error>"* %1 to %"std::result::Result<(), Error>::Ok"*
  %16 = bitcast %"std::result::Result<(), Error>::Ok"* %15 to {}*
  %17 = bitcast %"std::result::Result<(), Error>"* %1 to i32*
  store i32 2, i32* %17, align 8
  %18 = bitcast %"std::result::Result<(), Error>"* %1 to i128*
  %19 = load i128, i128* %18, align 8
  ret i128 %19

bb4:                                              ; preds = %start
  %20 = bitcast %"std::result::Result<(), Error>"* %self to i32*
  %21 = load i32, i32* %20, align 8, !range !13
  %22 = sub i32 %21, 2
  %23 = icmp eq i32 %22, 0
  %_2 = select i1 %23, i64 0, i64 1
  %24 = icmp eq i64 %_2, 0
  br i1 %24, label %bb1, label %bb2
}
