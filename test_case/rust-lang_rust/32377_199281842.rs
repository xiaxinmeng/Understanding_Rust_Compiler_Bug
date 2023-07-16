
INFO:rustc::middle::traits: type_known_to_meet_builtin_bound(ty=Bar<isize>, bound=Sized)
INFO:rustc::middle::traits::select: evaluate_obligation_conservatively(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: evaluate_predicate_recursively(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: evaluate_obligation_recursively(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: candidate_from_obligation(cache_fresh_trait_pred=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)), obligation=TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0)))
INFO:rustc::middle::traits::select: is_knowable(intercrate=false)
INFO:rustc::middle::traits::select: builtin_bound: bound=Sized
INFO:rustc::middle::traits::select: assemble_candidates_for_projected_tys(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: assemble_candidates_from_caller_bounds(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: candidate list size: 1
INFO:rustc::middle::traits::select: assembled 1 candidates for TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0)): [BuiltinCandidate(Sized)]
INFO:rustc::middle::traits::select: CACHE MISS: SELECT(Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)))=Ok(Some(BuiltinCandidate(Sized)))
INFO:rustc::middle::traits::select: evaluate_candidate: depth=0 candidate=BuiltinCandidate(Sized)
INFO:rustc::middle::traits::select: confirm_candidate(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0), BuiltinCandidate(Sized))
INFO:rustc::middle::traits::select: confirm_builtin_candidate(Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0))
INFO:rustc::middle::traits::select: vtable_builtin_data(obligation=Obligation(predicate=Binder(TraitPredicate(<Bar<isize> as core::marker::Sized>)),depth=0), bound=Sized, nested=Binder([core::marker::PhantomData<<isize as Foo>::Error>]))
INFO:rustc::middle::traits::select: select(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: candidate_from_obligation(cache_fresh_trait_pred=Binder(TraitPredicate(<isize as Foo>)), obligation=TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1)))
INFO:rustc::middle::traits::select: is_knowable(intercrate=false)
INFO:rustc::middle::traits::select: assemble_candidates_from_impls(obligation=Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: assemble_candidates_from_object_ty(self_ty=isize)
INFO:rustc::middle::traits::select: assemble_candidates_for_projected_tys(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: assemble_candidates_from_caller_bounds(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: assemble_candidates_from_default_impls(self_ty=isize)
INFO:rustc::middle::traits::select: candidate list size: 0
INFO:rustc::middle::traits::select: assembled 0 candidates for TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1)): []
INFO:rustc::middle::traits::select: CACHE MISS: SELECT(Binder(TraitPredicate(<isize as Foo>)))=Err(Unimplemented)
INFO:rustc::middle::traits::select: vtable_builtin_data: obligations=[Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1), Obligation(predicate=Binder(TraitPredicate(<core::marker::PhantomData<_> as core::marker::Sized>)),depth=1)]
INFO:rustc::middle::traits::select: evaluate_predicate_recursively(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: evaluate_obligation_recursively(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1))
INFO:rustc::middle::traits::select: candidate_from_obligation(cache_fresh_trait_pred=Binder(TraitPredicate(<isize as Foo>)), obligation=TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1)))
INFO:rustc::middle::traits::select: CACHE HIT: SELECT(Binder(TraitPredicate(<isize as Foo>)))=Err(Unimplemented)
INFO:rustc::middle::traits::select: CACHE MISS: EVAL(Binder(<isize as Foo>))=EvaluatedToErr
INFO:rustc::middle::traits::select: evaluate_predicate_recursively(Obligation(predicate=Binder(TraitPredicate(<isize as Foo>)),depth=1)) = EvaluatedToErr
INFO:rustc::middle::traits::select: evaluate_candidate: depth=0 result=EvaluatedToErr
INFO:rustc::middle::traits::select: CACHE MISS: EVAL(Binder(<Bar<isize> as core::marker::Sized>))=EvaluatedToErr
INFO:rustc::middle::traits: type_known_to_meet_builtin_bound: ty=Bar<isize> bound=Sized => false
