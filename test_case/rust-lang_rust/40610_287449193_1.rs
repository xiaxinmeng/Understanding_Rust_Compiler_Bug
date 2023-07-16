
error: main function not found

coercion::try(expr(23: { }): () -> ())
coercion::try(expr(17: 1.0): f32 -> f32)
coercion::try(expr(19: &[1.0]): &[f32; 1] -> &[f32])
Success, coerced with DerefRef { autoderefs: 1, autoref: Some(Ref('_#2r, MutImmutable)), unsize: true } -> _
coercion::try(expr(20: f(&[1.0])): () -> _)
error[E0369]: binary operation `+` cannot be applied to type `()`
 --> 1.rs:3:5
  |
3 |     () + f(&[1.0]);
  |     ^^
  |
  = note: an implementation of `std::ops::Add` might be missing for `()`

coercion::try(expr(17: 1.0): f32 -> f32)
coercion::try(expr(19: &[1.0]): &[f32; 1] -> &[f32])
Success, coerced with DerefRef { autoderefs: 1, autoref: Some(Ref('_#8r, MutImmutable)), unsize: true } -> _
error: internal compiler error: /Users/kennytm/XCodeProjects/rust/rust/src/librustc_typeck/check/coercion.rs:693: expr already has an adjustment on it!

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'Box<Any>', /Users/kennytm/XCodeProjects/rust/rust/src/librustc_errors/lib.rs:418
note: Run with `RUST_BACKTRACE=1` for a backtrace.
