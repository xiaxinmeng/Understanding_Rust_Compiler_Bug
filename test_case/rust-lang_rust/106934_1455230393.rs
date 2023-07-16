plain
    Checking cranelift-module v0.92.0
    Checking cranelift-frontend v0.92.0
    Checking cranelift-object v0.92.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no variant or associated item named `OffsetOf` found for enum `Rvalue` in the current scope
    |
    |
814 |                 Rvalue::OffsetOf(ref fields, container_ty) => {
    |                         ^^^^^^^^ variant or associated item not found in `Rvalue<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:01:58
