plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: hidden lifetime parameters in types are deprecated
  --> compiler/rustc_codegen_ssa/src/back/metadata.rs:98:49
   |
98 | fn create_object_file(sess: &Session) -> Option<write::Object> {
   |                                                 ^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
