plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `constness`
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:276:9
    |
276 |         constness: hir::Constness,
    |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_constness`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:42
