
┐rustc_trait_selection::traits::query::evaluate_obligation::evaluate_obligation obligation=Obligation(predicate=Binder(TraitPredicate(<std::future::from_generator::GenFuture<[static gener
ator@src/main.rs:586:28: 589:2]> as std::marker::Send>, polarity:Positive), []), depth=9)
├─0ms DEBUG rustc_query_system::query::plumbing ty::query::get_query<evaluate_obligation>(key=Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVar
Info { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0) }), ReL
ateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })), []), Binder(OutlivesPredicate(S14, ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })), [])], rev
eal: UserFacing, constness: NotConst }, value: Binder(TraitPredicate(<std::future::from_generator::GenFuture<[static generator@src/main.rs:586:28: 589:2]> as std::marker::Send>, polarity:
Positive), []) } }, span=src/main.rs:291:9: 296:11 (#0))
├─0ms DEBUG rustc_traits::evaluate_obligation evaluate_obligation(canonical_goal=Canonical {
│     max_universe: U0,
│     variables: [
│         CanonicalVarInfo {
│             kind: Region(
│                 U0,
│             ),
│         },
│         CanonicalVarInfo {
│             kind: Region(
│                 U0,
│             ),
│         },
│     ],
│     value: ParamEnvAnd {
│         param_env: ParamEnv {
│             caller_bounds: [
│                 Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0) }), ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })),
 []),
│                 Binder(OutlivesPredicate(S14, ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1) })), []),
│             ],
│             reveal: UserFacing,
│             constness: NotConst,
│         },
│         value: Binder(TraitPredicate(<std::future::from_generator::GenFuture<[static generator@src/main.rs:586:28: 589:2]> as std::marker::Send>, polarity:Positive), []),
│     },
│ })
├─0ms DEBUG rustc_infer::infer::region_constraints created new region variable '_#0r in U0 with origin MiscVariable(no-location (#0))
├─0ms DEBUG rustc_infer::infer::region_constraints created new region variable '_#1r in U0 with origin MiscVariable(no-location (#0))
├─0ms DEBUG rustc_traits::evaluate_obligation evaluate_obligation: goal=ParamEnvAnd {
│     param_env: ParamEnv {
│         caller_bounds: [
│             Binder(OutlivesPredicate('_#0r, '_#1r), []),
│             Binder(OutlivesPredicate(S14, '_#1r), []),
│         ],
│         reveal: UserFacing,
│         constness: NotConst,
│     },
│     value: Binder(TraitPredicate(<std::future::from_generator::GenFuture<[static generator@src/main.rs:586:28: 589:2]> as std::marker::Send>, polarity:Positive), []),
│ }
