llvm
define void @_ZN8rust_out3foo17hb77808e3fc28588aE(void ([128 x i64]*)* nocapture, void (i32)* nocapture, i32 ([128 x i64]*)* nocapture, i32* noalias dereferenceable(4)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %_19 = alloca [128 x i64], align 8
  %_13 = alloca [128 x i64], align 8
  %4 = bitcast [128 x i64]* %_13 to i8*
  call void @llvm.lifetime.start(i64 1024, i8* %4)
  invoke void %0([128 x i64]* noalias nocapture nonnull sret dereferenceable(1024) %_13)
          to label %bb3 unwind label %bb1

bb1:                                              ; preds = %entry-block, %bb3, %bb4, %bb5, %bb6, %bb7
  %5 = landingpad { i8*, i32 }
          cleanup
  %6 = bitcast i32* %3 to i8*
  tail call void @__rust_deallocate(i8* %6, i64 4, i64 4) #1
  resume { i8*, i32 } %5

bb3:                                              ; preds = %entry-block
  %7 = invoke i32 %2([128 x i64]* noalias nocapture nonnull dereferenceable(1024) %_13)
          to label %bb4 unwind label %bb1

bb4:                                              ; preds = %bb3
  call void @llvm.lifetime.end(i64 1024, i8* %4)
  invoke void %1(i32 %7)
          to label %bb5 unwind label %bb1

bb5:                                              ; preds = %bb4
  %8 = bitcast [128 x i64]* %_19 to i8*
  call void @llvm.lifetime.start(i64 1024, i8* %8)
  invoke void %0([128 x i64]* noalias nocapture nonnull sret dereferenceable(1024) %_19)
          to label %bb6 unwind label %bb1

bb6:                                              ; preds = %bb5
  %9 = invoke i32 %2([128 x i64]* noalias nocapture nonnull dereferenceable(1024) %_19)
          to label %bb7 unwind label %bb1

bb7:                                              ; preds = %bb6
  call void @llvm.lifetime.end(i64 1024, i8* %8)
  invoke void %1(i32 %9)
          to label %bb9 unwind label %bb1

bb9:                                              ; preds = %bb7
  %10 = bitcast i32* %3 to i8*
  tail call void @__rust_deallocate(i8* %10, i64 4, i64 4) #1
  ret void
}
