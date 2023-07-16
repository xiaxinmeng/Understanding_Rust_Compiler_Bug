plain
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0599]: `impl IntoIterator<Item = Vec<(Span, String)>>` is not an iterator
    |
    |
720 |         suggestions: impl IntoIterator<Item = Vec<(Span, String)>>,
    |                      --------------------------------------------- method `collect` not found for this type parameter
...
723 |         let suggestions: Vec<_> = suggestions.collect();
    |                                               ^^^^^^^ `impl IntoIterator<Item = Vec<(Span, String)>>` is not an iterator
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `impl IntoIterator<Item = Vec<(Span, String)>>: Iterator`
            which is required by `&mut impl IntoIterator<Item = Vec<(Span, String)>>: Iterator`
    = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `collect`, perhaps you need to restrict type parameter `impl IntoIterator<Item = Vec<(Span, String)>>` with it:
    |
720 |         suggestions: impl IntoIterator<Item = Vec<(Span, String)>> + Iterator,

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
