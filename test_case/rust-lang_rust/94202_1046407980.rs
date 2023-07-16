plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: passing `ty::consts::Const<'tcx>` by reference
  --> compiler/rustc_middle/src/ty/consts.rs:51:16
   |
51 |     pub fn val(&self) -> &ConstKind<'tcx> {
   |                ^^^^^ help: try passing by value: `ty::consts::Const<'tcx>`
   |
   = note: `-D rustc::pass-by-value` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:07:03
