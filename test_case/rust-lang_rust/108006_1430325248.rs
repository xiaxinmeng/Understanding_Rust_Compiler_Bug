plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0533]: expected unit struct, unit variant or constant, found struct variant `DefKind::Impl`
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2264:56
     |
2264 |                 if matches!(tcx.def_kind(id.owner_id), DefKind::Impl)
     |                                                        ^^^^^^^^^^^^^ not a unit struct, unit variant or constant
For more information about this error, try `rustc --explain E0533`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to previous error
