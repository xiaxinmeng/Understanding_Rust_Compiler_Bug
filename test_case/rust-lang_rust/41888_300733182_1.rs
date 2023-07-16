llvm
define internal void @_ZN5test24main17hd4f6b2920169b984E() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %infix_or_postfix.i = alloca %E, align 8         ; alloca a stack slot
  %0 = bitcast %E* %infix_or_postfix.i to i8*      ; for lifetime.end later on
  %1 = getelementptr inbounds %E, %E* %infix_or_postfix.i, i64 0, i32 0  ; discriminant
  %2 = load i64, i64* %1, align 8, !noalias !2     ; read the discriminant
  %cond11.i = icmp eq i64 %2, 0 ; …
  br i1 %cond11.i, label %_ZN5test21g17h3e85430d2d279929E.exit, label %bb3.i21.i ; russian roulette at this point

bb3.i21.i:                                        ; preds = %start
  %3 = getelementptr inbounds %E, %E* %infix_or_postfix.i, i64 0, i32 2
  %4 = bitcast [2 x i64]* %3 to %X*
  call fastcc void @_ZN4core3ptr13drop_in_place17h1a08b1322ab8b0ffE(%X* nonnull %4)
  br label %_ZN5test21g17h3e85430d2d279929E.exit

_ZN5test21g17h3e85430d2d279929E.exit:             ; preds = %bb3.i21.i, %start
  call void @llvm.lifetime.end(i64 24, i8* nonnull %0), !noalias !2
  ret void
