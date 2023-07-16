plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: unexpected `cfg` condition name
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2679:48
     |
2679 |     if unstable_self_contained_linker && !cfg!(rust_lld_enabled) {
     |
     |
     = note: `-D unexpected-cfgs` implied by `-D warnings`
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
