plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0070]: invalid left-hand side of assignment
    --> compiler/rustc_builtin_macros/src/deriving/generic/mod.rs:1863:17
     |
1862 | /             *sp.with_ctxt()
1863 | |             *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::Transparent);
     | |               - ^
     |                 cannot assign to this expression

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> compiler/rustc_builtin_macros/src/deriving/generic/mod.rs:1862:17
    --> compiler/rustc_builtin_macros/src/deriving/generic/mod.rs:1862:17
     |
1862 |             *sp.with_ctxt()
     |                 |
     |                 expected 1 argument
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_span/src/lib.rs:525:12
     |
525  |     pub fn with_ctxt(self, ctxt: SyntaxContext) -> Span {

error[E0614]: type `rustc_span::Span` cannot be dereferenced
    --> compiler/rustc_builtin_macros/src/deriving/generic/mod.rs:1862:13
     |
     |
1862 |             *sp.with_ctxt()

Some errors have detailed explanations: E0061, E0070, E0614.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_builtin_macros` due to 3 previous errors
