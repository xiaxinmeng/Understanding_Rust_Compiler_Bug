
~/Downloads $ rustc test.rs
test.rs:7:19: 7:30 warning: use of deprecated item: No longer needed, #[warn(deprecated)] on by default
test.rs:7 pub trait IsA<T>: MarkerTrait + PhantomFn<T> { }
                            ^~~~~~~~~~~
test.rs:7:33: 7:45 warning: use of deprecated item: No longer needed, #[warn(deprecated)] on by default
test.rs:7 pub trait IsA<T>: MarkerTrait + PhantomFn<T> { }
                                          ^~~~~~~~~~~~
test.rs:1:1: 1:1 error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(core::marker::Sized)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(Wrapper)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(IsA<_>)),depth=1),Ambiguity), FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(<_ as Wrapper>::Inner, i8)),depth=2),Ambiguity)]` fulfilling during trans
test.rs:1 use std::marker::{MarkerTrait, PhantomFn};
          ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:149
