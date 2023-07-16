plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0609]: no field `txc` on type `&DocContext<'_>`
   |
   |
79 |         || cx.txc.hir().span(hir_id).in_derive_expansion()
   |
   |
   = note: available fields are: `tcx`, `resolver`, `param_env`, `external_traits`, `active_extern_traits` ... and 11 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:03
