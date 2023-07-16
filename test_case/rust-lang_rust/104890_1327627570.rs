plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error: associated function `report_object_unsafe_cast` is never used
   --> compiler/rustc_hir_typeck/src/cast.rs:742:8
    |
742 |     fn report_object_unsafe_cast(&self, fcx: &FnCtxt<'a, 'tcx>, did: DefId) {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to previous error
Build completed unsuccessfully in 0:03:25
