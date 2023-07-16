plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
62 |             tcx.sess.span_err(attr.span, &format!("symbol-name({})", mangled));
   |
note: the lint level is defined here
  --> compiler/rustc_symbol_mangling/src/lib.rs:95:9
   |
   |
95 | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   |
   |
62 |             tcx.sess.span_err(attr.span, &format!("symbol-name({})", mangled));
   |
note: the lint level is defined here
  --> compiler/rustc_symbol_mangling/src/lib.rs:94:9
   |
   |
94 | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
64 |                 tcx.sess.span_err(attr.span, &format!("demangling({})", demangling));


error: diagnostics should be created using translatable messages
   |
   |
64 |                 tcx.sess.span_err(attr.span, &format!("demangling({})", demangling));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
65 |                 tcx.sess.span_err(attr.span, &format!("demangling-alt({:#})", demangling));


error: diagnostics should be created using translatable messages
   |
   |
65 |                 tcx.sess.span_err(attr.span, &format!("demangling-alt({:#})", demangling));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   |
   |
71 |             tcx.sess.span_err(attr.span, &format!("def-path({})", path));


error: diagnostics should be created using translatable messages
   |
   |
71 |             tcx.sess.span_err(attr.span, &format!("def-path({})", path));

error: could not compile `rustc_symbol_mangling` due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_symbol_mangling` due to 8 previous errors
