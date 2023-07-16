plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no function or associated item named `erase_self_ty` found for struct `ExistentialTraitRef` in the current scope
   --> compiler/rustc_middle/src/ty/vtable.rs:100:69
    |
100 |                     .map_bound(|trait_ref| ty::ExistentialTraitRef::erase_self_ty(tcx, trait_ref));
    |                                                                     ^^^^^^^^^^^^^ function or associated item not found in `ExistentialTraitRef<'_>`
   ::: compiler/rustc_middle/src/ty/sty.rs:901:1
    |
    |
901 | pub struct ExistentialTraitRef<'tcx> {
    | ------------------------------------ function or associated item `erase_self_ty` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
