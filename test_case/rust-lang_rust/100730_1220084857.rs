plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_monomorphize/src/collector.rs:933:44
    |
933 |                         let mut err = lint.build(&format!("moving {} bytes", layout.size.bytes()));
    |
note: the lint level is defined here
   --> compiler/rustc_monomorphize/src/lib.rs:7:9
    |
    |
7   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   --> compiler/rustc_monomorphize/src/collector.rs:933:44
    |
933 |                         let mut err = lint.build(&format!("moving {} bytes", layout.size.bytes()));
    |
note: the lint level is defined here
   --> compiler/rustc_monomorphize/src/lib.rs:6:9
    |
    |
6   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_monomorphize/src/polymorphize.rs:209:28
    |
209 |     let mut err = tcx.sess.struct_span_err(fn_span, "item has unused generic parameters");


error: diagnostics should be created using translatable messages
   --> compiler/rustc_monomorphize/src/polymorphize.rs:209:28
    |
209 |     let mut err = tcx.sess.struct_span_err(fn_span, "item has unused generic parameters");

error: could not compile `rustc_monomorphize` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_monomorphize` due to 4 previous errors
