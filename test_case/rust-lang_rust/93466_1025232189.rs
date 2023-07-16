plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0308]: mismatched types
   --> compiler/rustc_passes/src/dead.rs:263:63
    |
263 | ...                   self.ignored_derived_traits.entry(&adt_def_id)
    |                                                         ^^^^^^^^^^^ expected struct `LocalDefId`, found `&LocalDefId`
help: consider removing the borrow
    |
    |
263 -                             self.ignored_derived_traits.entry(&adt_def_id)
263 +                             self.ignored_derived_traits.entry(adt_def_id)

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
