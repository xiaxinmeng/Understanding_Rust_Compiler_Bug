
   Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `Tr + 'static: std::marker::Sized` is not satisfied
 --> src/main.rs:5:1
  |
5 | fn f(x: [Tr; 1]) {}
  | ^^^^^^^^^^^^^^^^^^^ `Tr + 'static` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Tr + 'static`
  = note: slice and array elements must have `Sized` type

error[E0038]: the trait `Tr` cannot be made into an object
 --> src/main.rs:5:1
  |
5 | fn f(x: [Tr; 1]) {}
  | ^^^^^^^^^^^^^^^^ the trait `Tr` cannot be made into an object
  |
  = note: method `foo` has no receiver
  = note: method `bar` has no receiver

error: internal compiler error: /checkout/src/librustc/ty/sty.rs:1372: Ty::fn_sig() called on non-fn type: [type error]

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0 (766bd11c8 2018-01-01) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:471:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
