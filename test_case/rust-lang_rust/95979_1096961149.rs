plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `def_id`
   --> compiler/rustc_trait_selection/src/traits/coherence.rs:792:21
    |
792 |         ty::Closure(def_id, ..) => {
    |                     ^^^^^^ help: if this is intentional, prefix it with an underscore: `_def_id`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
Build completed unsuccessfully in 0:02:09
