ll
define void @_ZN9borrowmut7bm_once17h7c1ef392d62ede16E(%"core::cell::RefCell<i32>"* dereferenceable(8)) unnamed_addr #1 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %1 = getelementptr inbounds %"core::cell::RefCell<i32>", %"core::cell::RefCell<i32>"* %0, i32 0, i32 0, i32 0, i32 0
  %2 = load i32, i32* %1, align 4, !noalias !8
  %cond.i.i.i = icmp eq i32 %2, 0
  br i1 %cond.i.i.i, label %"_ZN37_$LT$core..cell..RefCell$LT$T$GT$$GT$10borrow_mut17hd3c0713653e53ffbE.exit", label %bb4.i5.i

bb4.i5.i:                                         ; preds = %entry-block
  tail call fastcc void @_ZN4core6result13unwrap_failed17hd254683cc02cf089E(), !noalias !13
  unreachable

"_ZN37_$LT$core..cell..RefCell$LT$T$GT$$GT$10borrow_mut17hd3c0713653e53ffbE.exit": ; preds = %entry-block
  store i32 0, i32* %1, align 4
  ret void
}
