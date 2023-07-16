
┐rustc_trait_selection::traits::select::evaluate_predicate_recursively obligation=Obligation(predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), depth=5), previous_stack=Some(TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), depth=4)))
├─0ms DEBUG rustc_trait_selection::traits::select evaluate_trait_predicate_recursively, obligation=Obligation(predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), depth=5)
├─0ms DEBUG rustc_trait_selection::traits::select evaluate_trait_predicate_recursively - in global
├─0ms DEBUG rustc_trait_selection::traits::select fresh_trait_ref=Binder(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>, [])
├─0ms DEBUG rustc_trait_selection::traits::select get_provisional = None, fresh_trait_ref=Binder(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>, []), reached_depth=2
├─0ms DEBUG rustc_trait_selection::traits::select evaluate_stack --> recursive at depth 5
├─0ms DEBUG rustc_trait_selection::traits::select update_reached_depth, reached_depth=5
├─0ms DEBUG rustc_trait_selection::traits::select update_reached_depth: marking as cycle participant, p.fresh_trait_ref=Binder(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>, [])
├─0ms DEBUG rustc_trait_selection::traits::select coinductive_predicate, predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), result=true
├─0ms DEBUG rustc_trait_selection::traits::select evaluate_stack --> recursive, coinductive: EvaluatedToOk
├─0ms DEBUG rustc_trait_selection::traits::select result=Ok(EvaluatedToOk)
┘rustc_trait_selection::traits::select::evaluate_predicate_recursively obligation=Obligation(predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), depth=5), previous_stack=Some(TraitObligationStack(Obligation(predicate=Binder(TraitPredicate(<std::marker::PhantomData<proc_macro2::TokenTree> as std::marker::Unpin>), []), depth=4)))
