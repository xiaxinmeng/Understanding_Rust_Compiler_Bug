plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0061]: this method takes 2 arguments but 4 arguments were supplied
   --> src/common.rs:486:25
    |
486 |             self.0.sess.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)
    |                         ^^^^^^^^^^                                            --  --- unexpected argument of type `rustc_middle::ty::layout::LayoutError<'tcx>`
    |                                                                               unexpected argument of type `rustc_middle::ty::Ty<'tcx>`
    |
note: method defined here
   --> /checkout/compiler/rustc_session/src/session.rs:475:12
   --> /checkout/compiler/rustc_session/src/session.rs:475:12
    |
475 |     pub fn span_fatal<S: Into<MultiSpan>>(&self, sp: S, msg: impl Into<DiagnosticMessage>) -> ! {
help: remove the extra arguments
    |
    |
486 -             self.0.sess.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)
486 +             self.0.sess.span_fatal(span, "failed to get layout for `{}`: {}")

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_codegen_cranelift` (lib) due to previous error
Build completed unsuccessfully in 0:01:49
