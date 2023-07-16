
define zeroext i1 @_ZN8unrolled2g217h3aa4d16eeface44bE() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %0 = tail call i8* @__rust_allocate(i64 12, i64 4) #3
  %1 = icmp eq i8* %0, null
  br i1 %1, label %bb5.i, label %_ZN4drop17h3bfa678d62ddc26bE.exit

bb5.i:                                            ; preds = %entry-block
  tail call void @_ZN5alloc3oom3oom17h087c259ea35c365cE()
  unreachable

_ZN4drop17h3bfa678d62ddc26bE.exit:                ; preds = %entry-block
  %2 = bitcast i8* %0 to i32*
  store i32 1, i32* %2, align 4
  %3 = getelementptr inbounds i8, i8* %0, i64 4
  %4 = bitcast i8* %3 to i32*
  store i32 2, i32* %4, align 4
  %5 = getelementptr inbounds i8, i8* %0, i64 8
  %6 = bitcast i8* %5 to i32*
  store i32 3, i32* %6, align 4
  tail call void @__rust_deallocate(i8* nonnull %0, i64 12, i64 4) #3
  ret i1 true
}
