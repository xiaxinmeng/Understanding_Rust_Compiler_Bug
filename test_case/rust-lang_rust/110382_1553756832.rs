plain
    Checking idna v0.2.0
    Checking diff v0.1.13
    Checking bytes v1.0.1
    Checking rustc-workspace-hack v1.0.0
error[E0004]: non-exhaustive patterns: `rustc_ast::ExprKind::Matches(_, _, _)` not covered
     |
181  |         match expr.kind {
181  |         match expr.kind {
     |               ^^^^^^^^^ pattern `rustc_ast::ExprKind::Matches(_, _, _)` not covered
note: `rustc_ast::ExprKind` defined here
    --> /checkout/compiler/rustc_ast/src/ast.rs:1493:5
     |
1389 | pub enum ExprKind {
1389 | pub enum ExprKind {
     | -----------------
...
1493 |     Matches(P<Expr>, P<Arm>, P<Arm>),
     = note: the matched value is of type `rustc_ast::ExprKind`
     = note: the matched value is of type `rustc_ast::ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
263  ~             ),
263  ~             ),
264  ~             rustc_ast::ExprKind::Matches(_, _, _) => todo!(),

    Checking anstyle-wincon v0.2.0
    Checking concolor-query v0.3.3
    Checking filetime v0.2.20
