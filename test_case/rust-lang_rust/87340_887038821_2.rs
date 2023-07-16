
error: at least one trait must be specified
 --> mutant.rs:1:28
  |
1 | type PartiallyDefined<T> = impl 'static;
  |                            ^^^^^^^^^^^^

error[E0658]: `impl Trait` in type aliases is unstable
 --> mutant.rs:1:28
  |
1 | type PartiallyDefined<T> = impl 'static;
  |                            ^^^^^^^^^^^^
  |
  = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
  = help: add `#![feature(min_type_alias_impl_trait)]` to the crate attributes to enable

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: UnresolvedTy(_#0t)', compiler/rustc_typeck/src/check/writeback.rs:499:75
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (9c25eb7aa 2021-07-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type staticlib

query stack during panic:
#0 [typeck] type-checking `partially_defined`
#1 [fn_sig] computing function signature of `partially_defined`
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
