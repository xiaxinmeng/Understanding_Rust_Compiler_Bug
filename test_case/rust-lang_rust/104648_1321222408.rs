plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0433]: failed to resolve: use of undeclared type `DefKind`
   --> compiler/rustc_trait_selection/src/traits/object_safety.rs:884:50
    |
884 |             && tcx.def_kind(proj.item_def_id) == DefKind::ImplTraitPlaceholder
    |                                                  ^^^^^^^ use of undeclared type `DefKind`
[RUSTC-TIMING] rustc_parse test:false 26.808
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
[RUSTC-TIMING] rustc_save_analysis test:false 8.784
For more information about this error, try `rustc --explain E0433`.
