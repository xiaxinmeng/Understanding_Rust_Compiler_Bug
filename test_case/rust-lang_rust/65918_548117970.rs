
[DEBUG rustc::traits::query::normalize] QueryNormalizer: c_data = Canonical {
        max_universe: U0,
        variables: [],
        value: ParamEnvAnd {
            param_env: ParamEnv {
                caller_bounds: [
                    Binder(
                        TraitPredicate(<U as std::marker::Sized>),
                    ),
                    Binder(
                        TraitPredicate(<T as MyFrom<Phantom2<DummyT::<U>>>>),
                    ),
                    Binder(
                        TraitPredicate(<T as std::marker::Sized>),
                    ),
                ],
                reveal: All,
                def_id: None,
            },
            value: ProjectionTy {
                substs: [
                    T,
                    Phantom2<()>,
                ],
                item_def_id: DefId(0:19 ~ tait_normalize[317d]::MyFrom[0]::Error[0]),
            },
        },
    }
[DEBUG rustc::traits::query::normalize] QueryNormalizer: orig_values = OriginalQueryValues {
        universe_map: [
            U0,
        ],
        var_values: [],
    }
[DEBUG rustc::traits::project] opt_normalize_projection_type(projection_ty=ProjectionTy { substs: [T, Phantom2<()>], item_def_id: DefId(0:19 ~ tait_normalize[317d]::MyFrom[0]::Error[0]) }, depth=0)
[DEBUG rustc::traits::project] project(obligation=Obligation(predicate=ProjectionTy { substs: [T, Phantom2<()>], item_def_id: DefId(0:19 ~ tait_normalize[317d]::MyFrom[0]::Error[0]) }, depth=0))
[DEBUG rustc::traits::project] project: obligation_trait_ref=<T as MyFrom<Phantom2<()>>>
[DEBUG rustc::traits::project] assemble_candidates_from_param_env(..)
[DEBUG rustc::traits::project] assemble_candidates_from_predicates(obligation=Obligation(predicate=ProjectionTy { substs: [T, Phantom2<()>], item_def_id: DefId(0:19 ~ tait_normalize[317d]::MyFrom[0]::Error[0]) }, depth=0))
[DEBUG rustc::traits::project] assemble_candidates_from_predicates: predicate=Binder(TraitPredicate(<U as std::marker::Sized>))
[DEBUG rustc::traits::project] assemble_candidates_from_predicates: predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<DummyT::<U>>>>))
[DEBUG rustc::traits::project] assemble_candidates_from_predicates: predicate=Binder(TraitPredicate(<T as std::marker::Sized>))
[DEBUG rustc::traits::project] assemble_candidates_from_trait_def(..)
[DEBUG rustc::traits::select] select(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::select] candidate_from_obligation(cache_fresh_trait_pred=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), obligation=TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0)))
[DEBUG rustc::traits::select] is_knowable(intercrate=None)
[DEBUG rustc::traits::select] assemble_candidates_for_trait_alias(self_ty=T)
[DEBUG rustc::traits::select] assemble_candidates_from_impls(obligation=Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::select] assemble_candidates_from_object_ty(self_ty=T)
[DEBUG rustc::traits::select] assemble_candidates_for_projected_tys(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::select] assemble_candidates_from_caller_bounds(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::select] match_poly_trait_ref: obligation=Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0) poly_trait_ref=Binder(<T as MyFrom<Phantom2<DummyT::<U>>>>)
[DEBUG rustc::traits::select] assemble_candidates_from_auto_impls(self_ty=T)
[DEBUG rustc::traits::select] candidate list size: 0
[DEBUG rustc::traits::select] assembled 0 candidates for TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0)): []
[DEBUG rustc::traits::select] winnowed to 0 candidates for TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0)): []
[DEBUG rustc::traits::select] CACHE MISS: SELECT(Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)))=Err(Unimplemented)
[DEBUG rustc::traits::select] insert_candidate_cache(trait_ref=<T as MyFrom<Phantom2<()>>>, candidate=Err(Unimplemented)) local
[DEBUG rustc::traits::project] assemble_candidates_from_impls: selection error Unimplemented
[DEBUG rustc::traits::project] opt_normalize_projection_type: ERROR
[DEBUG rustc::traits::fulfill] register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::fulfill] select(obligation-forest-size=1)
[DEBUG rustc::traits::fulfill] select: starting another iteration
[DEBUG rustc::traits::fulfill] process_obligation: obligation = Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0) cause = ObligationCause { span: tait_normalize.rs:1:1: 1:1, body_id: HirId { owner: DefIndex(0), local_id: 4294967040 }, code: MiscObligation }
[DEBUG rustc::traits::select] select(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0))
[DEBUG rustc::traits::select] candidate_from_obligation(cache_fresh_trait_pred=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), obligation=TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0)))
[DEBUG rustc::traits::select] CACHE HIT: SELECT(Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)))=Err(Unimplemented)
[INFO  rustc::traits::fulfill] selecting trait `Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>))` at depth 0 yielded Err
[DEBUG rustc::traits::fulfill] select: outcome=Outcome {
        completed: None,
        errors: [
            Error {
                error: Unimplemented,
                backtrace: [
                    PendingPredicateObligation {
                        obligation: Obligation(predicate=Binder(TraitPredicate(<T as MyFrom<Phantom2<()>>>)), depth=0),
                        stalled_on: [],
                    },
                ],
            },
        ],
        stalled: false,
    }
[DEBUG rustc::traits::fulfill] select: starting another iteration
[DEBUG rustc::traits::fulfill] select: outcome=Outcome {
        completed: None,
        errors: [],
        stalled: true,
    }
[DEBUG rustc::traits::fulfill] select(0 predicates remaining, 1 errors) done
