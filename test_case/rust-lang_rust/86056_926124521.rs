plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0609]: no field `modules` on type `&rustc_hir::Crate<'tcx>`
  --> compiler/rustc_middle/src/hir/mod.rs:25:14
   |
25 |         self.modules.hash_stable(hcx, hasher);
   |
   |
   = note: available fields are: `owners`, `bodies`, `trait_map`, `attrs`
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
