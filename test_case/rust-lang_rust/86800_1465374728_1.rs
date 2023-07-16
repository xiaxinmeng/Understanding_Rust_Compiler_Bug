
error: internal compiler error: compiler/rustc_hir_typeck/src/inherited.rs:130:13: escaping bound vars in predicate Obligation(predicate=Binder(TraitPredicate(<TransactionFuture<'_, O> as std::marker::Sized>, polarity:Positive), []), depth=0)
 --> ice_min.rs:2:43
  |
2 | fn execute_transaction_fut<'f, F, O>() -> FnOnce(&Transaction) -> TransactionFuture<'_, O> {
  |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
