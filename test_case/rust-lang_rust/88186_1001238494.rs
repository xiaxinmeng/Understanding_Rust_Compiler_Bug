plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `rustc_rayon::iter::IndexedParallelIterator`
   --> compiler/rustc_middle/src/hir/map/mod.rs:527:13
    |
527 |         use rustc_rayon::iter::IndexedParallelIterator;
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:32
