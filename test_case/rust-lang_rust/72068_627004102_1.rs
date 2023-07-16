
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted(self_ty=Indexable, m_name=index_mut, trait_def_id=DefId(2:2150 ~ core[4753]::ops[0]::index[0]::IndexMut[0]))
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted: substs=[Indexable, &std::string::String]
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted: obligation=Obligation(predicate=Binder(TraitPredicate(<Indexable as std::ops::IndexMut<&std::string::String>>)), depth=0) trait_ref=<Indexable as std::ops::IndexMut<&std::string::String>>
[DEBUG rustc_typeck::check::method] --> Cannot match obligation
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted(self_ty=Indexable, m_name=index, trait_def_id=DefId(2:2146 ~ core[4753]::ops[0]::index[0]::Index[0]))
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted: substs=[Indexable, &std::string::String]
[DEBUG rustc_typeck::check::method] lookup_in_trait_adjusted: obligation=Obligation(predicate=Binder(TraitPredicate(<Indexable as std::ops::Index<&std::string::String>>)), depth=0) trait_ref=<Indexable as std::ops::Index<&std::string::String>>
[DEBUG rustc_typeck::check::method] --> Cannot match obligation
