plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unreachable pattern
   --> compiler/rustc_codegen_llvm/src/back/write.rs:134:9
    |
134 |         RelocModel::Pic | RelocModel::Pie => llvm::RelocModel::PIC,
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:11
