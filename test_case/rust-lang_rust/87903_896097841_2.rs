
│ │ │ │ │ ├─┐rustc_trait_selection::traits::select::match_impl impl_def_id=DefId(0:12 ~ issue_87803[87c4]::{impl#0}), obligation=Obligation(predicate=Binder(TraitPredicate(<IdScanner as Scanner>), []), cause=ObligationCauseData { span: no-location (#0), body_id: HirId { owner: DefId(0:0 ~ issue_87803[87c4]), local_id: 0 }, code: MiscObligation }, param_env=ParamEnv { caller_bounds: [], reveal: UserFacing }, depth=0)
│ │ │ │ │ │ ├─0ms DEBUG rustc_infer::infer::higher_ranked replace_bound_vars_with_placeholders(next_universe=U1, result=TraitPredicate(<IdScanner as Scanner>), map={})
│ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::select impl_trait_ref=<IdScanner as Scanner>
│ │ │ │ │ │ ├┐
│ │ │ │ │ │ ├─┐rustc_trait_selection::traits::project::normalize_with_depth_to depth=1, value=<IdScanner as Scanner>
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::project obligations.len=0
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::project value=<IdScanner as Scanner>
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::project result=<IdScanner as Scanner>, obligations.len=0
│ │ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::project normalizer.obligations=[]
│ │ │ │ │ │ │┌┘
│ │ │ │ │ │ ├┘
│ │ │ │ │ │ ├─0ms DEBUG rustc_trait_selection::traits::select impl_trait_ref=<IdScanner as Scanner>, placeholder_obligation_trait_ref=<IdScanner as Scanner>
