
DEBUG:rustc_typeck::check::wfcheck: check_item_well_formed(it.id=73, it.name=foo)
DEBUG:rustc::traits::fulfill: select(obligation-forest-size=0)
DEBUG:rustc::traits::fulfill: select: starting another iteration
DEBUG:rustc::traits::fulfill: select: outcome=Outcome { completed: [], errors: [], stalled: true }
DEBUG:rustc::traits::fulfill: select(0 predicates remaining, 0 errors) done
DEBUG:rustc_typeck::check: normalize_associated_types_in(value=Binder(([]; variadic: false)->(T, T)))
DEBUG:rustc_typeck::check: normalize_associated_types_in: result=Binder(([]; variadic: false)->(T, T)) predicates=[]
DEBUG:rustc_typeck::check: normalize_associated_types_in(value=InstantiatedPredicates([Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<Self as std::clone::Clone>)), Binder(TraitPredicate(<Self as std::default::Default>))]))
DEBUG:rustc_typeck::check: normalize_associated_types_in: result=InstantiatedPredicates([Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<Self as std::clone::Clone>)), Binder(TraitPredicate(<Self as std::default::Default>))]) predicates=[]
DEBUG:rustc_typeck::check::wfcheck: check_item_fn: predicates=InstantiatedPredicates([Binder(TraitPredicate(<T as std::marker::Sized>)), Binder(TraitPredicate(<Self as std::clone::Clone>)), Binder(TraitPredicate(<Self as std::default::Default>))])
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 1
DEBUG:rustc_typeck::check: normalize_associated_types_in(value=Binder(([]; variadic: false)->(T, T)))
DEBUG:rustc_typeck::check: normalize_associated_types_in: result=Binder(([]; variadic: false)->(T, T)) predicates=[]
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 2
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 3
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 4
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 5
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=WF((T, T)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=WF((T, T)),depth=0))
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 6
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 7
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=WF(T),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=WF(T),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=Binder(TraitPredicate(<Self as std::clone::Clone>)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<Self as std::clone::Clone>)),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=WF(Self),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=WF(Self),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=Binder(TraitPredicate(<Self as std::default::Default>)),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=Binder(TraitPredicate(<Self as std::default::Default>)),depth=0))
DEBUG:rustc_typeck::check: register_predicate(Obligation(predicate=WF(Self),depth=0))
DEBUG:rustc::traits::fulfill: register_predicate_obligation(obligation=Obligation(predicate=WF(Self),depth=0))
DEBUG:rustc_typeck::check::wfcheck: check_item_fn_or_method: 8
DEBUG:rustc_typeck::check: select_all_obligations_or_error
DEBUG:rustc::traits::fulfill: select(obligation-forest-size=7)
DEBUG:rustc::traits::fulfill: select: starting another iteration
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<T as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::clone::Clone>))` at depth 0 yielded Ok(Some)
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::default::Default>))` at depth 0 yielded Ok(Some)
DEBUG:rustc::traits::fulfill: select: outcome=Outcome { completed: [PendingPredicateObligation { obligation: Obligation(predicate=Binder(TraitPredicate(<Self as std::default::Default>)),depth=0), stalled_on: [] }, PendingPredicateObligation { obligation: Obligation(predicate=WF(Self),depth=0), stalled_on: [] }, PendingPredicateObligation { obligation: Obligation(predicate=Binder(TraitPredicate(<Self as std::clone::Clone>)),depth=0), stalled_on: [] }, PendingPredicateObligation { obligation: Obligation(predicate=WF(T),depth=0), stalled_on: [] }], errors: [], stalled: false }
DEBUG:rustc::traits::fulfill: select: starting another iteration
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<T as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: select: outcome=Outcome { completed: [], errors: [], stalled: true }
DEBUG:rustc::traits::fulfill: select(3 predicates remaining, 0 errors) done
DEBUG:rustc::traits::fulfill: select(obligation-forest-size=3)
DEBUG:rustc::traits::fulfill: select: starting another iteration
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<T as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: select: outcome=Outcome { completed: [], errors: [], stalled: true }
DEBUG:rustc::traits::fulfill: select(3 predicates remaining, 0 errors) done
DEBUG:rustc::traits::fulfill: select(obligation-forest-size=3)
DEBUG:rustc::traits::fulfill: select: starting another iteration
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<T as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<T as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Self as std::marker::Sized>))` at depth 0 yielded Ok(None)
DEBUG:rustc::traits::fulfill: process_predicate: pending obligation Obligation(predicate=Binder(TraitPredicate(<Self as std::marker::Sized>)),depth=0) now stalled on []
DEBUG:rustc::traits::fulfill: select: outcome=Outcome { completed: [], errors: [], stalled: true }
DEBUG:rustc::traits::fulfill: select(3 predicates remaining, 0 errors) done
error[E0282]: type annotations needed
  --> src/test/run-pass/trait-alias.rs:27:1
   |
27 | / fn foo<T: CD>() -> (T, T) {
28 | |     let one = T::default();
29 | |     let two = one.clone();
30 | |     (one, two)
31 | | }
   | |_^ cannot infer type for `T`
