

bb111:                                            ; preds = %bb112
  store i8 0, i8* %tmp191
  cleanupret from %cleanuppad7 unwind label %bb110

bb112:                                            ; preds = %bb113, %bb113, %bb113
  %497 = load i8, i8* %tmp191, !range !3
  %498 = trunc i8 %497 to i1
  br i1 %498, label %bb111, label %bb112_cleanup_trampoline_bb110

bb113:                                            ; preds = %bb114
  %499 = getelementptr inbounds %"evm::interpreter::InstructionResult<usize>", %"evm::interpreter::InstructionResult<usize>"* %result, i32 0, i32 0
  %500 = load i64, i64* %499, !range !12
  switch i64 %500, label %bb113_cleanup_trampoline_bb110 [
    i64 0, label %bb112
    i64 1, label %bb112
    i64 6, label %bb112
  ]
