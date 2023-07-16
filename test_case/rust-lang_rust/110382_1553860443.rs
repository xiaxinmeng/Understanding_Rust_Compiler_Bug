plain
   Compiling rustfmt-config_proc_macro v0.3.0 (/checkout/src/tools/rustfmt/config_proc_macro)
   Compiling derive-new v0.5.8
    Checking thiserror v1.0.38
    Checking clap v3.2.20
error[E0004]: non-exhaustive patterns: `ExprKind::Matches(_, _, _)` not covered
     |
464  |     match expr.kind {
464  |     match expr.kind {
     |           ^^^^^^^^^ pattern `ExprKind::Matches(_, _, _)` not covered
note: `ExprKind` defined here
    --> /checkout/compiler/rustc_ast/src/ast.rs:1493:5
     |
1389 | pub enum ExprKind {
1389 | pub enum ExprKind {
     | -----------------
...
1493 |     Matches(P<Expr>, P<Arm>, P<Arm>),
     = note: the matched value is of type `ExprKind`
     = note: the matched value is of type `ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
512  ~         | ast::ExprKind::Underscore => false,
512  ~         | ast::ExprKind::Underscore => false,
513  ~         ExprKind::Matches(_, _, _) => todo!(),


error[E0004]: non-exhaustive patterns: `ExprKind::Matches(_, _, _)` not covered
     |
     |
68   |     let expr_rw = match expr.kind {
     |                         ^^^^^^^^^ pattern `ExprKind::Matches(_, _, _)` not covered
note: `ExprKind` defined here
    --> /checkout/compiler/rustc_ast/src/ast.rs:1493:5
     |
1389 | pub enum ExprKind {
1389 | pub enum ExprKind {
     | -----------------
...
1493 |     Matches(P<Expr>, P<Arm>, P<Arm>),
     = note: the matched value is of type `ExprKind`
     = note: the matched value is of type `ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
408  ~         ast::ExprKind::Err => None,
408  ~         ast::ExprKind::Err => None,
409  ~         ExprKind::Matches(_, _, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustfmt-nightly` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
