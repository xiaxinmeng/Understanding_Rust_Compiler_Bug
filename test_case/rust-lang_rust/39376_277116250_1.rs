
define i32 @foo(%Bar* noalias readonly dereferenceable(5)) unnamed_addr #0 !dbg !4 {
entry-block:
  %arg0 = alloca %Bar*
  %b = alloca %Bar*
  store %Bar* %0, %Bar** %arg0
  call void @llvm.dbg.declare(metadata %Bar** %arg0, metadata !20, metadata !21), !dbg !22
  call void @llvm.dbg.declare(metadata %Bar** %b, metadata !23, metadata !21), !dbg !25
  br label %start

start:                                            ; preds = %entry-block
  %1 = load %Bar*, %Bar** %arg0, !dbg !26, !nonnull !2
  store %Bar* %1, %Bar** %b, !dbg !26
  %2 = load %Bar*, %Bar** %b, !dbg !27, !nonnull !2
  %3 = getelementptr inbounds %Bar, %Bar* %2, i32 0, i32 1, !dbg !27
  %4 = getelementptr inbounds %Foo, %Foo* %3, i32 0, i32 0, !dbg !27
  %5 = load i32, i32* %4, !dbg !27 ; you would expect an "align 1" here
  ret i32 %5, !dbg !28
}
