rust
enum RepeatSyntax { Dyn, None }
syntax::ast::ExprKind::Repeat(P<Expr>, P<Expr>, RepeatSyntax)

enum RepeatExprCount {
  Const(BodyId),
  Dyn(P<Expr>),
}
hir::Expr_::ExprRepeat(P<Expr>, RepeatExprCount)
