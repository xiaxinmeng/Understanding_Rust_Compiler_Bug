plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:182:34
    |
182 | ...                   sess.span_err(item.span(), msg);
    |
note: the lint level is defined here
   --> compiler/rustc_metadata/src/lib.rs:19:9
    |
    |
19  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:182:34
    |
182 | ...                   sess.span_err(item.span(), msg);
    |
note: the lint level is defined here
   --> compiler/rustc_metadata/src/lib.rs:18:9
    |
    |
18  | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:187:34
    |
187 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:187:34
    |
187 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:192:34
    |
192 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:192:34
    |
192 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:205:38
    |
205 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:205:38
    |
205 | ...                   sess.span_err(item.span(), msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_metadata/src/native_libs.rs:305:26
    |
305 |                     sess.span_err(span, msg);


error: diagnostics should be created using translatable messages
   --> compiler/rustc_metadata/src/native_libs.rs:305:26
    |
305 |                     sess.span_err(span, msg);

error: could not compile `rustc_metadata` due to 10 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to 10 previous errors
