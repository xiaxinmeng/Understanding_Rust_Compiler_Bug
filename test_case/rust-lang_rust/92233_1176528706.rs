plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0433]: failed to resolve: could not find `impl_stable_hash_via_hash` in `rustc_data_structures`
    |
    |
198 | rustc_data_structures::impl_stable_hash_via_hash!(Level);
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `impl_stable_hash_via_hash` in `rustc_data_structures`
error: unused import: `rustc_hir::HashStableContext`
  --> compiler/rustc_lint_defs/src/lib.rs:13:5
   |
13 | use rustc_hir::HashStableContext;
13 | use rustc_hir::HashStableContext;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: `std::mem::Discriminant<Level>` is not an iterator
     |
     |
194  |         std::mem::discriminant(self).partial_cmp(std::mem::discriminant(other))
     |                                      ^^^^^^^^^^^ `std::mem::Discriminant<Level>` is not an iterator
    ::: /checkout/library/core/src/mem/mod.rs:1060:1
     |
     |
1060 | pub struct Discriminant<T>(<T as DiscriminantKind>::Discriminant);
     | ------------------------------------------------------------------ doesn't satisfy `std::mem::Discriminant<Level>: Iterator`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `std::mem::Discriminant<Level>: Iterator`
             which is required by `&mut std::mem::Discriminant<Level>: Iterator`
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `rustc_lint_defs` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
