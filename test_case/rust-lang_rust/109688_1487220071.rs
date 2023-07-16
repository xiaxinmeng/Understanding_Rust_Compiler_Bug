plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0425]: cannot find value `tcx` in this scope
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:576:53
    |
576 |         let mut v: Vec<CrateNum> = CStore::from_tcx(tcx)

error[E0425]: cannot find value `tcx` in this scope
error[E0425]: cannot find value `tcx` in this scope
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:581:21
581 |                     tcx
    |                     ^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
