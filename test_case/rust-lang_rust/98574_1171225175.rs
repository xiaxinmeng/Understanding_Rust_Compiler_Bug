plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0716]: temporary value dropped while borrowed
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:1313:35
     |
1313 |         let Some(liberated_sig) = typeck_results.liberated_fn_sigs().get(fn_hir_id) else { return false; };
     |                                   |
     |                                   creates a temporary which is freed while still in use
...
1346 |         match liberated_sig.output().kind() {
