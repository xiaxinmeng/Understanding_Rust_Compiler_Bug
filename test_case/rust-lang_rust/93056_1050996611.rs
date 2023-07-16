plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:523:28
    |
523 |             create_DIArray(DIB(cx), &signature_metadata[..]),
    |                            ^^^^^^^ expected struct `context::CodegenCx`, found struct `DIBuilder`
    = note: expected reference `&context::CodegenCx<'_, '_>`
               found reference `&DIBuilder<'_>`

For more information about this error, try `rustc --explain E0308`.
