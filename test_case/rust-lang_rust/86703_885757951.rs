
error: internal compiler error: broken MIR in DefId(0:13 ~ rust_out[6150]::main::_doctest_main_src_yoke_rs_488_0::slice::{closure#0}) (bb0[0]): equate_normalized_input_or_output: `&[closure@src/yoke.rs:8:14: 8:40]==&[closure@src/yoke.rs:8:14: 8:40]` failed with `NoSolution`
 --> src/yoke.rs:493:14
  |
8 |    y.project(move |yk, _| yk.as_bytes())
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:253:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
