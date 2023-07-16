plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: redundant field names in struct initialization
   --> compiler/rustc_codegen_llvm/src/type_of.rs:273:42
    |
273 |             TypeLowering { lltype: llty, field_remapping: field_remapping },
    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `field_remapping`
    |
    = note: `-D redundant-field-initializers` implied by `-D warnings`
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:07:26
