
$ RUSTC_LOG=rustc_typeck::check::closure=debug,rustc_trait_selection::traits::project=debug,rustc_trait_selection::traits::select::confirmation=debug rustc +stage1 ~/misc/rust/ice.rs
...
┐rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
├─0ms DEBUG rustc_trait_selection::traits::select::confirmation confirm_closure_candidate, obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0)
├─0ms DEBUG rustc_trait_selection::traits::select::confirmation confirm_closure_candidate before normalization, trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))])
├┐rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
│└┐rustc_trait_selection::traits::project::normalize_with_depth_to depth=1, value=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))])
│ ├─0ms DEBUG rustc_trait_selection::traits::project result=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))]), obligations.len=0
│ ├─0ms DEBUG rustc_trait_selection::traits::project normalizer.obligations=[]
│┌┘rustc_trait_selection::traits::project::normalize_with_depth_to depth=1, value=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))])
├┘rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
├─0ms DEBUG rustc_trait_selection::traits::select::confirmation confirm closure candidate obligations, closure_def_id=DefId(0:11 ~ ice[317d]::main::{closure#0}), trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))]), obligations=[]
├┐rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
│└┐rustc_trait_selection::traits::select::confirmation::confirm_poly_trait_refs obligation_cause=ObligationCauseData { span: no-location (#0), body_id: HirId { owner: DefId(0:0 ~ ice[317d]), local_id: 0 }, code: MiscObligation }, obligation_param_env=ParamEnv { caller_bounds: [], reveal: All }, obligation_trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>, []), expected_trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))])
│┌┘rustc_trait_selection::traits::select::confirmation::confirm_poly_trait_refs obligation_cause=ObligationCauseData { span: no-location (#0), body_id: HirId { owner: DefId(0:0 ~ ice[317d]), local_id: 0 }, code: MiscObligation }, obligation_param_env=ParamEnv { caller_bounds: [], reveal: All }, obligation_trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>, []), expected_trait_ref=Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))])
├┘rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
┘rustc_trait_selection::traits::select::confirmation::confirm_candidate obligation=Obligation(predicate=Binder(TraitPredicate(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>), []), depth=0), candidate=ClosureCandidate
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:78:17: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>, [Region(BrAnon(0))]), Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>, []), Sorts(ExpectedFound { expected: u64, found: <u64 as Viewable<'_>>::View }))` selecting `Binder(<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>, [])` during codegen
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1007:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-dev running on x86_64-apple-darwin

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::FnMut` fulfills its obligations
#1 [resolve_instance] resolving instance `<[closure@/Users/jorendorff/misc/rust/ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>::call_mut`
end of query stack
error: aborting due to previous error
