
warning: constant evaluation error: attempt to subtract with overflow. This will become a HARD ERROR in the future
 --> src/main.rs:3:20
  |
3 |     const X: u32 = 0-1;
  |                    ^^^
  |
  = note: #[warn(const_err)] on by default

error[E0080]: constant evaluation error
 --> src/main.rs:3:20
  |
3 |     const X: u32 = 0-1;
  |                    ^^^ attempt to subtract with overflow

error: internal compiler error: /checkout/src/librustc_trans/mir/constant.rs:378: _1 not initialized
 --> src/main.rs:4:20
  |
4 |     println!("{}", X);
  |                    ^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (f85579d4a 2017-07-12) running on x86_64-unknown-linux-gnu
