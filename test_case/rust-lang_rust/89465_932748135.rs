plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0599]: no method named `body_owners` found for struct `TyCtxt<'tcx>` in the current scope
    |
996 |         .body_owners()
    |          ^^^^^^^^^^^ method not found in `TyCtxt<'tcx>`

