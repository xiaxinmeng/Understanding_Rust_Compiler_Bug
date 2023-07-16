plain
    Checking smallvec v1.10.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#fe242b7e)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
482 |             self.tcx.sess.span_fatal(span, format!("failed to get layout for `{}`: {}", ty, err))
    |
note: the lint level is defined here
   --> src/lib.rs:22:9
    |
    |
22  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
482 |             self.tcx.sess.span_fatal(span, format!("failed to get layout for `{}`: {}", ty, err))
    |
note: the lint level is defined here
   --> src/lib.rs:21:9
    |
    |
21  | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_codegen_gcc` (lib) due to 2 previous errors
Build completed unsuccessfully in 0:01:51
