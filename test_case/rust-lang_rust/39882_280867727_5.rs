
define internal void @_ZN5hello4main17hf7b9630e95734ebdE() unnamed_addr #1 !dbg !34 {
entry-block:
  %arg = alloca { i32, {} }
  %_8 = alloca {}
  %_7 = alloca { i32, {} }
  %_2 = alloca {}
  %x = alloca { i32, {} }
  %_0 = alloca {}
  call void @llvm.dbg.declare(metadata { i32, {} }* %x, metadata !38, metadata !24), !dbg !40
  br label %start, !dbg !40

start:                                            ; preds = %entry-block
  %0 = getelementptr inbounds { i32, {} }, { i32, {} }* %x, i32 0, i32 0, !dbg !41
  store i32 24, i32* %0, !dbg !41
  %1 = getelementptr inbounds { i32, {} }, { i32, {} }* %_7, i32 0, i32 0, !dbg !42
  store i32 42, i32* %1, !dbg !42
  %2 = getelementptr inbounds { i32, {} }, { i32, {} }* %_7, i32 0, i32 0, !dbg !43
  %3 = getelementptr inbounds { i32, {} }, { i32, {} }* %_7, i32 0, i32 1, !dbg !43
  %4 = load i32, i32* %2, !dbg !43
  %5 = getelementptr inbounds { i32, {} }, { i32, {} }* %arg, i32 0, i32 0, !dbg !43
  store i32 %4, i32* %5, !dbg !43
  %6 = getelementptr inbounds { i32, {} }, { i32, {} }* %arg, i32 0, i32 1, !dbg !43
  store {} undef, {}* %6, !dbg !43
  %7 = bitcast { i32, {} }* %arg to i32*, !dbg !43
  %8 = load i32, i32* %7, align 4, !dbg !43
  call void @_ZN4core3ptr5write17h6c3e55ca0821fa79E({ i32, {} }* %x, i32 %8), !dbg !43
  br label %bb1, !dbg !43

bb1:                                              ; preds = %start
  ret void, !dbg !44
}
