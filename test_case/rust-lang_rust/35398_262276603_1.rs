llvm
%closure = type {}

; Function Attrs: uwtable
define internal void @test::main() unnamed_addr #0 {
entry-block:
  %_0 = alloca {}
  br label %start

start:                                            ; preds = %entry-block
  call void @test::main::function()
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: uwtable
define internal void @test::main::function() unnamed_addr #0 {
entry-block:
  %_0 = alloca {}
  %c = alloca %closure
  %_4 = alloca { i32 }
  br label %start

start:                                            ; preds = %entry-block
  %0 = bitcast { i32 }* %_4 to i8*
  call void @llvm.lifetime.start(i64 4, i8* %0)
  %1 = getelementptr inbounds { i32 }, { i32 }* %_4, i32 0, i32 0
  store i32 2, i32* %1
  %2 = load { i32 }, { i32 }* %_4
  %3 = extractvalue { i32 } %2, 0
  %4 = call i32 @"test::main::function::{{closure}}"(%closure* noalias readonly %c, i32 %3)
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast { i32 }* %_4 to i8*
  call void @llvm.lifetime.end(i64 4, i8* %5)
  ret void
}

; Function Attrs: uwtable
define internal i32 @"test::main::function::{{closure}}"(%closure* noalias readonly, i32) unnamed_addr #0 {
entry-block:
  br label %start

start:                                            ; preds = %entry-block
  %2 = add i32 %1, 4
  ret i32 %2
}
