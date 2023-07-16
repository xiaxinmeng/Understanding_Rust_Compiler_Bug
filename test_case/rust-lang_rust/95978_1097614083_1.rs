`
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:10 ~ issue_91636[2737]::main), const_param_did: None }) (end of phase transition to Optimized) at bb2[3]:
                                Cannot compare unequal types fn(&str) and for<'r> fn(&'r str)
  --> src/tools/miri/tests/run-pass/issue-91636.rs:19:13
   |
19 |     assert!(inner == func2.inner);
   |             ^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:126:36
