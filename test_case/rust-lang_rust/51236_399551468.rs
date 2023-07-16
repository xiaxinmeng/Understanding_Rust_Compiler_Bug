rust
$ RUST_LOG=rustc::traits::auto_trait rustdoc +local -o asdf l.rs
DEBUG 2018-06-22T19:03:59Z: rustc::traits::auto_trait: evaluate_nested_obligations(ty_did=DefId(0/0:7 ~ l[8787]::Owned[0]), trait_did=DefId(2/0:865 ~ core[1c6a]::marker[0]::Send[0])): succeeded with 'ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as traits::Owned<'a>>)), Binder(TraitPredicate(<T
as std::marker::Sized>)), Binder(TraitPredicate(<<T as traits::Owned<'static>>::Reader as std::marker::Send>)), Binder(TraitPredicate(<T as traits::Owned<'static>>))], reveal: UserFacing }' 'ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as traits::Owned<'static>>)), Binder(TraitPredicate(<<T as
traits::Owned<'static>>::Reader as std::marker::Send>)), Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<T as traits::Owned<'a>>))], reveal: UserFacing }'
DEBUG 2018-06-22T19:03:59Z: rustc::traits::auto_trait: evaluate_nested_obligations(ty_did=DefId(0/0:7 ~ l[8787]::Owned[0]), trait_did=DefId(2/0:865 ~ core[1c6a]::marker[0]::Send[0])): succeeded with 'ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as traits::Owned<'a>>)), Binder(TraitPredicate(<T
as std::marker::Sized>)), Binder(TraitPredicate(<<T as traits::Owned<'static>>::Reader as std::marker::Send>)), Binder(TraitPredicate(<T as traits::Owned<'static>>))], reveal: UserFacing }' 'ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as traits::Owned<'static>>)), Binder(TraitPredicate(<<T as
traits::Owned<'static>>::Reader as std::marker::Send>)), Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<T as traits::Owned<'a>>))], reveal: UserFacing }'
DEBUG 2018-06-22T19:03:59Z: rustc::traits::auto_trait: find_auto_trait_generics(did=DefId(0/0:7 ~ l[8787]::Owned[0]), trait_did=DefId(2/0:865 ~ core[1c6a]::marker[0]::Send[0]), generics=Generics { parent: None, parent_count: 0, params: [Type(T, DefId(0/1:10 ~ l[8787]::Owned[0]::T[0]), 0)], param_def_id_to_index: {DefId(0/1:10 ~ l[8787]::Owned[0]::T[0]): 0}, has_self: false, has_late_bound_regions: None }): fulfilling with ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as traits::Owned<'a>>)), Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<<T as traits::Owned<'static>>::Reader as std::marker::Send>)), Binder(TraitPredicate(<T as traits::Owned<'static>>))], reveal: UserFacing }
thread '<unnamed>' panicked at 'Unable to fulfill trait DefId(2/0:865 ~ core[1c6a]::marker[0]::Send[0]) for 'Owned<T>': [FulfillmentError(Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [T, ReStatic], item_def_id: DefId(0/0:6 ~ l[8787]::traits[0]::Owned[0]::Reader[0]) }, _)),depth=2),Ambiguity), FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<_ as std::marker::Send>)),depth=2),Ambiguity)]', librustc/traits/auto_trait.rs:218:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
