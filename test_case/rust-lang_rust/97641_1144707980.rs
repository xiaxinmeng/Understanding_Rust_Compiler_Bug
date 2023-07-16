plain
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unreachable pattern
   --> compiler/rustc_infer/src/infer/outlives/obligations.rs:264:13
    |
264 |             &ty::Projection(_) => {
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
[RUSTC-TIMING] rustc_infer test:false 3.243
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_symbol_mangling test:false 3.876
