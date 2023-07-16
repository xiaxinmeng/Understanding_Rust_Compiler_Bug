llvm
; thread_local::get_the_thread_local
; Function Attrs: uwtable
define i64 @_ZN12thread_local20get_the_thread_local17h9b1ad8597fd10cadE() unnamed_addr #4 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = load {}*, {}** bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h6da2233c675cfa90E to {}**), align 8, !alias.scope !51
  %1 = icmp eq {}* %0, null
  br i1 %1, label %_ZN12thread_local20get_the_thread_local1X7__getit17h0fb9551d9b330632E.exit.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17ha7d145a93e39ed07E.exit"

_ZN12thread_local20get_the_thread_local1X7__getit17h0fb9551d9b330632E.exit.i.i: ; preds = %start
; call std::thread::local::fast::Key<T>::try_initialize_drop
  %2 = tail call fastcc align 8 dereferenceable_or_null(24) i64* @"_ZN3std6thread5local4fast12Key$LT$T$GT$19try_initialize_drop17hdbbedea7d28cfb9fE"()
  %3 = icmp eq i64* %2, null
  br i1 %3, label %bb4.i.i, label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17ha7d145a93e39ed07E.exit"

bb4.i.i:                                          ; preds = %_ZN12thread_local20get_the_thread_local1X7__getit17h0fb9551d9b330632E.exit.i.i
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17h73420553132c6252E()
  unreachable

"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17ha7d145a93e39ed07E.exit": ; preds = %start, %_ZN12thread_local20get_the_thread_local1X7__getit17h0fb9551d9b330632E.exit.i.i
  %_0.0.i.i1.i.i = phi i64* [ %2, %_ZN12thread_local20get_the_thread_local1X7__getit17h0fb9551d9b330632E.exit.i.i ], [ bitcast (<{ [32 x i8] }>* @_ZN12thread_local20get_the_thread_local1X7__getit5__KEY17h6da2233c675cfa90E to i64*), %start ]
  %4 = getelementptr i64, i64* %_0.0.i.i1.i.i, i64 2
  %.idx.val.i.i = load i64, i64* %4, align 8, !alias.scope !54
  ret i64 %.idx.val.i.i
}
