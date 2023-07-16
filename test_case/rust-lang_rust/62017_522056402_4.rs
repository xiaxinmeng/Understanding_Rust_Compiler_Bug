
define void @_ZN4main8example217h4f443cbc5e17bbddE() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*,    %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_v = alloca %Droppable, align 1
  invoke void @_ZN4main5takep17h851b97fbf6407c1bE(void ()* nonnull @_ZN1a6voidfn17h7d790e74ed3ff940E)
          to label %bb2 unwind label %cleanup

bb2:                                              ; preds = %start
  call void @_ZN4core3ptr18real_drop_in_place17h6782efda6a962288E(%Droppable* nonnull align 1 %_v)
  br label %bb4

bb4:                                              ; preds = %bb2
  ret void
