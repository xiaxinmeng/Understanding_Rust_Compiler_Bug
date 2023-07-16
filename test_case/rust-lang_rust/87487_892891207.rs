plain
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
   Compiling tracing-tree v0.1.9
error: variant is never constructed: `Multiline`
  --> compiler/rustc_errors/src/snippet.rs:73:5
73 |     Multiline(MultilineAnnotation),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:06
