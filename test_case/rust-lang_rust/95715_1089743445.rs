plain
    Checking structopt v0.3.25
error[E0631]: type mismatch in function arguments
    --> src/tools/rustfmt/src/closures.rs:97:62
     |
97   |             if let Some(expr) = block.stmts.first().and_then(stmt_expr) {
     |                                                     -------- ^^^^^^^^^ expected signature of `fn(&P<rustc_ast::Stmt>) -> _`
     |                                                     required by a bound introduced by this call
     |
    ::: src/tools/rustfmt/src/utils.rs:322:1
     |
     |
322  | pub(crate) fn stmt_expr(stmt: &ast::Stmt) -> Option<&ast::Expr> {
     | --------------------------------------------------------------- found signature of `for<'r> fn(&'r rustc_ast::Stmt) -> _`
note: required by a bound in `std::option::Option::<T>::and_then`
    --> /checkout/library/core/src/option.rs:1248:12
     |
     |
1248 |         F: ~const FnOnce(T) -> Option<U>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::option::Option::<T>::and_then`
error[E0308]: mismatched types
   --> src/tools/rustfmt/src/closures.rs:150:21
    |
    |
150 |           stmts: vec![ast::Stmt {
    |  _____________________^
151 | |             id: ast::NodeId::root(),
152 | |             kind: ast::StmtKind::Expr(ptr::P(body.clone())),
153 | |             span: body.span,
154 | |         }],
    | |_________^ expected struct `P`, found struct `rustc_ast::Stmt`
    |
    = note: expected struct `P<rustc_ast::Stmt>`
               found struct `rustc_ast::Stmt`

error[E0271]: type mismatch resolving `<std::slice::Iter<'_, P<rustc_ast::Stmt>> as Iterator>::Item == &rustc_ast::Stmt`
    |
    |
898 |         self.walk_stmts(&Stmt::from_ast_nodes(b.stmts.iter()), false)
    |                          ^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_ast::Stmt`, found struct `P`
    = note: expected reference `&rustc_ast::Stmt`
    = note: expected reference `&rustc_ast::Stmt`
               found reference `&P<rustc_ast::Stmt>`
note: required by a bound in `stmt::Stmt::<'a>::from_ast_nodes`
    |
    |
40  |     pub(crate) fn from_ast_nodes<I>(iter: I) -> Vec<Self>
    |                   -------------- required by a bound in this
41  |     where
42  |         I: Iterator<Item = &'a ast::Stmt>,
    |                     ^^^^^^^^^^^^^^^^^^^^ required by this bound in `stmt::Stmt::<'a>::from_ast_nodes`
Some errors have detailed explanations: E0271, E0308, E0631.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `rustfmt-nightly` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
