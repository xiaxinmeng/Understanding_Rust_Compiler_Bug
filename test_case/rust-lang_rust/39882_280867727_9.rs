
define internal void @_ZN5hello4main17hf7b9630e95734ebdE() unnamed_addr #0 !dbg !5 {
entry-block:
  %y = alloca i32
  %x = alloca {}
  %_0 = alloca {}
  call void @llvm.dbg.declare(metadata {}* %x, metadata !10, metadata !13), !dbg !14
  call void @llvm.dbg.declare(metadata i32* %y, metadata !15, metadata !13), !dbg !18
  br label %start, !dbg !18

start:                                            ; preds = %entry-block
  %0 = bitcast {}* %x to i32*, !dbg !19
  %1 = load i32, i32* %0, !dbg !19
  store i32 %1, i32* %y, !dbg !19
  ret void, !dbg !20
}
