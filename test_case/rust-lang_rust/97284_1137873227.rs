plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused variable: `reg_vid`
   --> compiler/rustc_middle/src/ty/impls_ty.rs:138:23
    |
138 |             ty::ReVar(reg_vid) => {
    |                       ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_reg_vid`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:45
