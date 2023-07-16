plain
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0407]: method `query_crate` is not a member of trait `Key`
   --> compiler/rustc_query_impl/src/keys.rs:401:5
401 | /     fn query_crate(&self) -> CrateNum {
402 | |         LOCAL_CRATE
403 | |     }
    | |_____^ not a member of trait `Key`
    | |_____^ not a member of trait `Key`

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0046]: not all trait items implemented, missing: `query_crate_is_local`
   --> compiler/rustc_query_impl/src/keys.rs:400:1
    |
18  |     fn query_crate_is_local(&self) -> bool;
    |     --------------------------------------- `query_crate_is_local` from trait
...
400 | impl<'tcx> Key for (ty::Predicate<'tcx>, HirId) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `query_crate_is_local` in implementation
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0407.
