
error: internal compiler error: broken MIR in DefId(0/0:4 ~ foo[317d]::main[0]) (UserAssertTy(Canonical { variables: [], value: impl std::cmp::PartialEq<i32> }, _1)): bad type assert (Canonical { variables: [], value: impl std::cmp::PartialEq<i32> } = i32): NoSolution
 --> foo.rs:8:9
  |
8 |     let x: impl PartialEq<i32> = 123_i32;
  |         ^

thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', librustc_errors/lib.rs:322:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic
