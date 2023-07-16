plain
    Checking cranelift-frontend v0.95.1
    Checking cranelift-native v0.95.1
    Checking cranelift-object v0.95.1
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `span_fatal` found for reference `&common::RevealAllLayoutCx<'tcx>` in the current scope
    |
    |
486 |             self.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)
    |                  ^^^^^^^^^^ method not found in `&RevealAllLayoutCx<'tcx>`
help: one of the expressions' fields has a method of the same name
    |
    |
486 |             self.0.sess.span_fatal(span, "failed to get layout for `{}`: {}", ty, err)

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_cranelift` (lib) due to previous error
Build completed unsuccessfully in 0:02:15
