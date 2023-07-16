rust
declare_lint! {
    // we can bikeshed the lint name
    DUP_ASSOC_TYPE_BINDING,
    Warn,
    "warns about duplicate associated type bindings in generics, e.g. `Iterator<Item = (), Item = ()>`"
}

pub struct DupAssocTypeBinding;

impl LintPass for DupAssocTypeBinding {
    fn get_lints(&self) -> LintArray {
        lint_array!(DUP_ASSOC_TYPE_BINDING)
    }
}

impl EarlyLintPass for DupAssocTypeBinding {
    fn check_path(&self, ctxt: &EarlyContext, path: &ast::Path) {
        // could be `Iterator` or `std::iter::Iterator` or `Iterator<Item = ()>::next`
        // need to check all segments
        for segment in &path.segments {
            // if its `Iterator<..>` or `Iterator(...)` (for closure traits)
            if let Some(ref params) = segment.parameters {
                // if it's `Iterator<...>`
                if let ast::AngleBracketed(ref params_data) = &**params {
                    // in `params_data.bindings`, if two elements have the same 
                    // `ident` and `ty` fields but differing `id` fields,
                    // emit a lint message with:
                    ctxt.span_lint(DUP_ASSOC_TYPE_BINDING, 
                                   vec![param_span_1, param_span_2], 
                                   "duplicate associated type binding");
                }
            }
        }
    }
}
