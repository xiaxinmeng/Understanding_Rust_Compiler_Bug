plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unused import: `PanicStrategy`
  --> compiler/rustc_codegen_ssa/src/back/write.rs:35:42
   |
35 | use rustc_target::spec::{MergeFunctions, PanicStrategy, SanitizerSet};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
