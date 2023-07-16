plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs:691:16
    |
691 |         if let Some(hir_id) = self.path_segment.hir_id
    |                ^^^^^^^^^^^^   ------------------------ this expression has type `HirId`
    |                |
    |                expected struct `HirId`, found enum `Option`
    = note: expected struct `HirId`
                 found enum `Option<_>`

For more information about this error, try `rustc --explain E0308`.
