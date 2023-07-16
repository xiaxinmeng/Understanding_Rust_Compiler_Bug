plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused doc comment
   --> compiler/rustc_codegen_llvm/src/llvm/ffi.rs:579:1
    |
579 |   /// LLVMRustThinLTOData
580 | / extern "C" {
581 | |     pub type ThinLTOData;
582 | | }
582 | | }
    | |_- rustdoc does not generate documentation for extern block
    |
    = note: `-D unused-doc-comments` implied by `-D warnings`
    = help: use `//` for a plain comment
error: unused doc comment
   --> compiler/rustc_codegen_llvm/src/llvm/ffi.rs:584:1
    |
    |
584 |   /// LLVMRustThinLTOBuffer
585 | / extern "C" {
586 | |     pub type ThinLTOBuffer;
587 | | }
587 | | }
    | |_- rustdoc does not generate documentation for extern block
    |
    = help: use `//` for a plain comment
error: could not compile `rustc_codegen_llvm` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:42
