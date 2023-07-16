plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0599]: no method named `impl_trait_in_trait_parent_fn` found for struct `TyCtxt<'_>` in the current scope
   --> compiler/rustc_hir_analysis/src/check/check.rs:203:32
    |
203 |             let trait_fn = tcx.impl_trait_in_trait_parent_fn(id.owner_id.to_def_id());

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_hir_analysis` due to previous error
warning: build failed, waiting for other jobs to finish...
