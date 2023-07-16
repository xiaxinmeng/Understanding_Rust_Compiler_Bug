plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0308]: mismatched types
    --> compiler/rustc_ast_lowering/src/lib.rs:2119:17
     |
2119 |                 hir::GenericArgs::none(),
     |                 |
     |                 |
     |                 expected `&rustc_hir::GenericArgs<'_>`, found struct `rustc_hir::GenericArgs`
     |                 help: consider borrowing here: `&hir::GenericArgs::none()`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_ast_lowering`
