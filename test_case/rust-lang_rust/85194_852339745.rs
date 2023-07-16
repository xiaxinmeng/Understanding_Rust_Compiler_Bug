
error: argument to `panic!()` in a const context must have type `&str`
 --> src/main.rs:4:5
  |
4 |     panic!(123);
  |     ^^^^^^^^^^^^
  |
  = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at '`ref_to_mplace` called on non-ptr type', compiler/rustc_mir/src/interpret/place.rs:307:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#1 [eval_to_const_value_raw] simplifying constant for the type system `main::{constant#0}`
end of query stack
