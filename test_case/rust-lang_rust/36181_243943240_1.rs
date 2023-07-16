 llvm
define i64 @test(i32, i32) unnamed_addr #0 {
entry-block:
  %sret_slot = alloca %"2.std::option::Option<u32>"
  %return = alloca %"2.std::option::Option<u32>"
  br label %start

start:                                            ; preds = %entry-block
  %2 = icmp eq i32 %0, %1
  %3 = call zeroext i1 @llvm.expect.i1(i1 zeroext %2, true)
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %3, label %bb2, label %bb3

bb2:                                              ; preds = %bb1
  %4 = getelementptr inbounds %"2.std::option::Option<u32>", %"2.std::option::Option<u32>"* %return, i32 0, i32 0
  store i32 0, i32* %4
  br label %bb5

bb3:                                              ; preds = %bb1
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %0, i32 %1)
  %6 = extractvalue { i32, i1 } %5, 0
  %7 = extractvalue { i32, i1 } %5, 1
  %8 = call i1 @llvm.expect.i1(i1 %7, i1 false)
  br i1 %8, label %panic, label %bb4

bb4:                                              ; preds = %bb3
  %9 = getelementptr inbounds %"2.std::option::Option<u32>", %"2.std::option::Option<u32>"* %return, i32 0, i32 0
  store i32 1, i32* %9
  %10 = bitcast %"2.std::option::Option<u32>"* %return to { i32, i32 }*
  %11 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %10, i32 0, i32 1
  store i32 %6, i32* %11
  br label %bb5

bb5:                                              ; preds = %bb2, %bb4
  %12 = bitcast %"2.std::option::Option<u32>"* %return to i64*
  %13 = load i64, i64* %12, align 4
  ret i64 %13

panic:                                            ; preds = %bb3
  call void @_ZN4core9panicking5panic17h44f94ad2f4e3e170E({ %str_slice, %str_slice, i32 }* @panic_loc7544)
  unreachable
}
