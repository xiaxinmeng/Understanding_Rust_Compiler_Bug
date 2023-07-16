plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: local binding shadows glob re-export
   --> compiler/rustc_middle/src/ty/mod.rs:148:1
    |
73  | pub use rustc_type_ir::*;
    |         ---------------- the name `sty` in the type namespace is introduced by the glob reexport here
...
148 | mod sty;
    | ^^^^^^^^ but the local binding here shadows the name `sty` in the type namespace
    |
    = note: `-D local-binding-shadows-glob-reexport` implied by `-D warnings`
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: could not compile `rustc_middle` (lib) due to previous error
