
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:4 ~ playground[f128]::main)) (after phase change to runtime-optimized) at bb1[1]:
                                Cannot offset by non-isize type bool
 --> src/main.rs:9:21
  |
9 |         assert_eq!(*offset(ptr, true), 1);
  |                     ^^^^^^^^^^^^^^^^^
  |
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
