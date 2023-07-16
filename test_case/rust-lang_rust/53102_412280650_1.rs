
$ rustdoc --version
rustdoc 1.28.0 (9634041f0 2018-07-30)
$ rustdoc lib.rs
thread '<unnamed>' panicked at 'Unable to fulfill trait DefId(2/0:865 ~ core[537d]::marker[0]::Send[0]) for 'Owned<T>': [FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [T, ReStatic], item_def_id: DefId(0/0:6 ~ lib[8787]::OwnedTrait[0]::Reader[0]) }, _)),depth=2),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<_ as std::marker::Send>)),depth=2),Ambiguity)]', librustc/traits/auto_trait.rs:218:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0 (9634041f0 2018-07-30) running on x86_64-apple-darwin
