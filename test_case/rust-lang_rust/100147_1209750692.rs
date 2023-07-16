plain
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_privacy/src/lib.rs:405:31
    |
405 |                 self.tcx.sess.span_fatal(
    |
note: the lint level is defined here
   --> compiler/rustc_privacy/src/lib.rs:9:34
    |
    |
9   | #![cfg_attr(not(bootstrap), deny(rustc::diagnostic_outside_of_impl))]


error: diagnostics should be created using translatable messages
   --> compiler/rustc_privacy/src/lib.rs:405:31
    |
405 |                 self.tcx.sess.span_fatal(
    |
note: the lint level is defined here
   --> compiler/rustc_privacy/src/lib.rs:8:34
    |
    |
8   | #![cfg_attr(not(bootstrap), deny(rustc::untranslatable_diagnostic))]

error: could not compile `rustc_privacy` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:08:17
