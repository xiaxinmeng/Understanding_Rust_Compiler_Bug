
error[E0308]: mismatched types
  --> src\tools\miri\src\bin\miri.rs:42:61
   |
42 |             if let Some(return_code) = miri::eval_main(tcx, entry_def_id, config) {
   |                                                             ^^^^^^^^^^^^ expected struct `rustc_hir::def_id::DefId`, found struct `rustc_hir::def_id::LocalDefId`

error: aborting due to previous error
