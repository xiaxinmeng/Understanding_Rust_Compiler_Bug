plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0599]: `std::mem::Discriminant<Level>` is not an iterator
     |
     |
183  |         std::mem::discriminant(self).partial_cmp(std::mem::discriminant(other))
     |                                      ^^^^^^^^^^^ `std::mem::Discriminant<Level>` is not an iterator
    ::: /checkout/library/core/src/mem/mod.rs:1038:1
     |
     |
1038 | pub struct Discriminant<T>(<T as DiscriminantKind>::Discriminant);
     | ------------------------------------------------------------------ doesn't satisfy `std::mem::Discriminant<Level>: Iterator`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `std::mem::Discriminant<Level>: Iterator`
             which is required by `&mut std::mem::Discriminant<Level>: Iterator`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lint_defs` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint_defs` due to previous error
