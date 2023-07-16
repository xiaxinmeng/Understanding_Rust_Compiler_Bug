plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: usage of `ty::TyKind::<kind>`
   --> compiler/rustc_middle/src/ty/diagnostics.rs:156:13
    |
156 |             TyKind::Param(param) => param.name.as_str().starts_with("impl"),
    |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`
    |
    = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:06:15
