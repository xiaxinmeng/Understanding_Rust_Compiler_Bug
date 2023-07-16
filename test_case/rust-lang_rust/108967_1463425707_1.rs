
warning: unused variable: `val`
 --> src/main.rs:4:9
  |
4 |     let val = std::intrinsics::wrapping_sub(&1, &2);
  |         ^^^ help: if this is intentional, prefix it with an underscore: `_val`
  |
  = note: `#[warn(unused_variables)]` on by default

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3 ~ playground[e443]::main), const_param_did: None }) (after phase change to runtime-optimized) at bb0[2]:
                                Cannot perform arithmetic on type &i32
 --> src/main.rs:4:15
  |
4 |     let val = std::intrinsics::wrapping_sub(&1, &2);
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
...
