plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: unused import: `ParallelIterator`
 --> compiler/rustc_passes/src/hir_id_validator.rs:2:41
  |
2 | use rustc_data_structures::sync::{Lock, ParallelIterator};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:44
