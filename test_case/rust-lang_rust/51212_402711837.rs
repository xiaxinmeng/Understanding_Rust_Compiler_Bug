rust
fn lookup_import_candidates_from_module(
    &mut self, 
    lookup_name: Name, 
    namespace: Namespace, 
    start_module: Module<'_>,
    filter_fn: impl Fn(Def) -> bool,
) -> Vec<ImportSuggestion> {
    ...
}
