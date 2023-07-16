plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: unused imports: `CRATE_DEF_INDEX`, `LOCAL_CRATE`
  --> compiler/rustc_privacy/src/lib.rs:15:39
   |
15 | use rustc_hir::def_id::{CRATE_DEF_ID, CRATE_DEF_INDEX, LOCAL_CRATE};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: mismatched types
   --> compiler/rustc_privacy/src/lib.rs:590:44
    |
    |
590 |             Option::<AccessLevel>::of_impl(item.hir_id(), self.tcx, &self.access_levels)
    |                                            ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_privacy` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
