
error[E0308]: mismatched types
 --> src/main.rs:2:16
  |
2 |     [9; || [9; []]];
  |                ^^ expected `usize`, found array of 0 elements
  |
  = note: expected type `usize`
            found array `[_; 0]`

error: internal compiler error: compiler/rustc_const_eval/src/interpret/eval_context.rs:198:17: The type checker should prevent reading from a never-written local

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1171:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
