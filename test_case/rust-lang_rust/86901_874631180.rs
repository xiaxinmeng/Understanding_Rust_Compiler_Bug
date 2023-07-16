plain
    Checking url v2.2.2
error[E0599]: no method named `type_implements_trait` found for struct `TyCtxt<'tcx>` in the current scope
   --> src/tools/clippy/clippy_utils/src/ty.rs:132:10
    |
132 |         .type_implements_trait(trait_id, ty, ty_params, cx.param_env)
    | 
   ::: /checkout/compiler/rustc_trait_selection/src/infer.rs:50:8
    |
50  |     fn type_implements_trait(
50  |     fn type_implements_trait(
    |        --------------------- the method is available for `TyCtxt<'tcx>` here
    |
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::rustc_trait_selection::infer::TyCtxTraitsExt;`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_utils`
