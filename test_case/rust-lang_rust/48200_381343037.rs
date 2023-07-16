llvm
; issue_48200::foo
; Function Attrs: norecurse nounwind readnone uwtable
define i64 @_ZN11issue_482003foo17h7f3d764b4e2d0d3cE(i64 %a.0, i64 %a.1, i64 %b.0, i64 %b.1) unnamed_addr #0 {
start:
  %cond = icmp eq i64 %a.0, 1
  %cond1 = icmp eq i64 %b.0, 1
  %or.cond = and i1 %cond, %cond1
  %0 = add i64 %b.1, %a.1
  %_0.0 = select i1 %or.cond, i64 %0, i64 0
  ret i64 %_0.0
}

; issue_48200::bar
; Function Attrs: uwtable
define i64 @_ZN11issue_482003bar17he38cd8ae2cebf23eE(i64, i64, i64, i64) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %switch.i = icmp eq i64 %0, 1
  %switch.i2 = icmp eq i64 %2, 1
  %or.cond = and i1 %switch.i, %switch.i2
  %4 = add i64 %3, %1
  %_0.0 = select i1 %or.cond, i64 %4, i64 0
  ret i64 %_0.0
}

; issue_48200::baz
; Function Attrs: norecurse nounwind readnone uwtable
define i64 @_ZN11issue_482003baz17h789590496be40da2E(i64, i64, i64, i64) unnamed_addr #0 {
start:
  %cond = icmp eq i64 %0, 1
  %cond1 = icmp eq i64 %2, 1
  %or.cond = and i1 %cond, %cond1
  %4 = add i64 %3, %1
  %_0.0 = select i1 %or.cond, i64 %4, i64 0
  ret i64 %_0.0
}
