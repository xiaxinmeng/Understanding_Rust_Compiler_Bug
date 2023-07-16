plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0599]: the method `partial_cmp` exists for struct `std::mem::Discriminant<Level>`, but its trait bounds were not satisfied
     |
     |
182  |         std::mem::discriminant(self).partial_cmp(std::mem::discriminant(other))
     |                                      ^^^^^^^^^^^ method cannot be called on `std::mem::Discriminant<Level>` due to unsatisfied trait bounds
    ::: /checkout/library/core/src/mem/mod.rs:1032:1
     |
     |
1032 | pub struct Discriminant<T>(<T as DiscriminantKind>::Discriminant);
     | ------------------------------------------------------------------ doesn't satisfy `std::mem::Discriminant<Level>: Iterator`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `std::mem::Discriminant<Level>: Iterator`
             which is required by `&mut std::mem::Discriminant<Level>: Iterator`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lint_defs` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
