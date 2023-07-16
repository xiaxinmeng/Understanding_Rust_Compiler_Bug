plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0433]: failed to resolve: use of undeclared type `DefKind`
   --> compiler/rustc_trait_selection/src/traits/object_safety.rs:884:50
    |
884 |             && tcx.def_kind(proj.item_def_id) == DefKind::ImplTraitPlaceholder
    |                                                  ^^^^^^^ use of undeclared type `DefKind`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
