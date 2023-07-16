plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/project.rs:506:40
    |
506 |                     .unwrap_or_else(|| ty.super_fold_with(self))
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_middle::ty::Term`, found `&TyS<'_>`
help: try wrapping the expression in `rustc_middle::ty::Term::Ty`
    |
    |
506 |                     .unwrap_or_else(|| rustc_middle::ty::Term::Ty(ty.super_fold_with(self)))
    |                                        +++++++++++++++++++++++++++                        +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
