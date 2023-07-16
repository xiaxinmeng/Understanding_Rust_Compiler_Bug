plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0609]: no field `def_kind` on type `rmeta::TableBuilders<'tcx>`
   --> compiler/rustc_metadata/src/rmeta/encoder.rs:986:33
    |
986 |             record!(self.tables.def_kind[def_id] <- def_kind);
    |
    |
    = note: available fields are: `kind`, `attributes`, `children`, `opt_def_kind`, `visibility` ... and 36 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
