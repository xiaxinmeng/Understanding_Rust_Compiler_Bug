rust
error: no method named `f` found for type `{integer}` in the current scope
 --> src/lib.rs:5:11
  |
5 |         3.f()
  |           ^
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'byte index 4 is out of bounds of ``', src/libcore/str/mod.rs:2162
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `a`.
