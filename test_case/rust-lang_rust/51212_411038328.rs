rust
    fn lookup_import_candidates<FilterFn>(&mut self,
                                          lookup_name: Name,
                                          namespace: Namespace,
                                          filter_fn: FilterFn)
                                          -> Vec<ImportSuggestion>
        where FilterFn: Fn(Def) -> bool
    {
        let mut suggestions = vec![];

        suggestions.extend(
            self.lookup_import_candidates_from_module(
                lookup_name, namespace, self.graph_root, keywords::Crate.name(), filter_fn
            )
        );

        if extern_prelude_feature_is_enabled {
            let extern_prelude_names = self.extern_prelude.clone();
            for &krate_name in extern_prelude_names.iter() {
                let extern_prelude_module = self.load_extern_prelude_crate_if_needed(krate_name);
                // ^^ that is the new helper method we added

                suggestions.extend(
                    self.lookup_import_candidates_from_module(
                        lookup_name, namespace, extern_prelude_module, krate_name, filter_fn
                    )
                );
            }
        }
    }
