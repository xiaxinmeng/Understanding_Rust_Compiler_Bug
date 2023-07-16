plain
[RUSTC-TIMING] rustc_session test:false 5.785
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_ast_passes test:false 4.458
error[E0594]: cannot assign to `self.span`, as `self` is not declared as mutable
  --> compiler/rustc_middle/src/ty/query.rs:98:9
   |
97 |     pub fn at(self, span: Span) -> TyCtxtAt<'tcx> {
   |               ---- help: consider changing this to be mutable: `mut self`
98 |         self.span = span;

[RUSTC-TIMING] rustc_expand test:false 9.536
For more information about this error, try `rustc --explain E0594`.
[RUSTC-TIMING] rustc_middle test:false 14.165
