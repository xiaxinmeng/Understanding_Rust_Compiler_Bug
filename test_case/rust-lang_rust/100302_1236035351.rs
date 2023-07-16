plain
   |
11 | use rustc_hir::def::DefKind;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: associated function `check_for_deref_method` is never used
     |
1641 |     fn check_for_deref_method(
     |        ^^^^^^^^^^^^^^^^^^^^^^
     |
     |
     = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to 2 previous errors
Build completed unsuccessfully in 0:02:29
