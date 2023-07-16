
warning: unused import: `GeneratorState`
 --> src/main.rs:3:27
  |
3 | use std::ops::{Generator, GeneratorState};
  |                           ^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: internal compiler error: /checkout/src/librustc_typeck/check/mod.rs:632: escaping regions in predicate Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: Slice([_]), item_def_id: DefId { krate: CrateNum(2), node: DefIndex(911) => core/bdc9c6a::ops[0]::generator[0]::Generator[0]::Yield[0] } }, &T)),depth=0)
  --> src/main.rs:11:31
   |
11 | fn walk<T>(tree: &Tree<T>) -> impl Generator<Yield=&T, Return=()> {
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-nightly (c11f689d2 2017-08-29) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:443:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
